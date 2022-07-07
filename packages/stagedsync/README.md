# stagedsync package
Staged Sync is a version of Go-Ethereum's Full Sync that was rearchitected for better performance.

Staged Sync, as its name suggests, consists of 10 stages that are executed in order, one after another.

## How staged sync work?

For each peer Erigon learns what the HEAD blocks is and it executes each stage in order for the missing blocks between the local HEAD block and the peer's head blocks.

The first stage (downloading headers) sets the local HEAD block.

Each stage is executed in order and a stage N does not stop until the local head is reached for it.

That mean, that in the ideal scenario (no network interruptions, the app isn't restarted, etc), for the full initial sync, each stage will be executed exactly once.

After the last stage is finished, the process starts from the beginning, by looking for the new headers to download.

If the app is restarted in between stages, it restarts from the first stage.

If the app is restarted in the middle of the stage execution, it restarts from that stage, giving it the opportunity to complete.

## Stages
Each stage consists of 2 functions `ExecFunc` that progesses the stage forward and `UnwindFunc` that unwinds the stage backwards.

Most of the stages can work offline though it isn't implemented in the current version.
We can add/remove stages, so exact stage numbers may change - but order and names stay the same.

```rust
pub const HEADERS: StageId = StageId("Headers");
pub const BLOCK_HASHES: StageId = StageId("BlockHashes");
pub const BODIES: StageId = StageId("Bodies");
pub const SENDERS: StageId = StageId("SenderRecovery");
pub const TOTAL_GAS_INDEX: StageId = StageId("TotalGasIndex");
pub const TOTAL_TX_INDEX: StageId = StageId("TotalTxIndex");
pub const EXECUTION: StageId = StageId("Execution");
pub const INTERMEDIATE_HASHES: StageId = StageId("IntermediateHashes");
pub const HASH_STATE: StageId = StageId("HashState");
pub const ACCOUNT_HISTORY_INDEX: StageId = StageId("AccountHistoryIndex");
pub const STORAGE_HISTORY_INDEX: StageId = StageId("StorageHistoryIndex");
pub const LOG_INDEX: StageId = StageId("LogIndex");
pub const CALL_TRACES: StageId = StageId("CallTraces");
pub const TX_LOOKUP: StageId = StageId("TxLookup");
pub const TX_POOL: StageId = StageId("TxPool");
pub const FINISH: StageId = StageId("Finish");
```


### Stage `Headers`

During this stage we download all the headers between the local HEAD and our peer's head.

This stage is CPU intensive and can benefit from a multicore processor due to verifying PoW of the headers.

Most of the unwinds are initiated on this stage due to the chain reorgs.

This stage promotes local HEAD pointer.

### Stage `BlockHashes`

Creates an index of blockHash -> blockNumber extracted from the headers for faster lookups and making the sync friendlier for HDDs.

### Stage `Bodies`

At that stage, we download bodies for block headers that we already downloaded.

That is the most intensive stage for the network connection, the vast majority of data is downloaded here.

### Stage `SenderRecovery`

This stage recovers and stores senders for each transaction in each downloaded block.

This is also a CPU intensive stage and also benefits from multi-core CPUs.

This stage doesn't use any network connection.

### Stage `TotalGasIndex`
TODO

### Stage `TotalTxIndex`
TODO

### Stage 7: [Execute Blocks Stage](/eth/stagedsync/stage_execute.go)

During this stage, we execute block-by-block everything that we downloaded before.

One important point there, that we don't check root hashes during this execution, we don't even build a merkle trie here.

This stage is single threaded.

This stage doesn't use internet connection.

This stage is disk intensive.

This stage can spawn unwinds if the block execution fails.

### Stage `Execution`
TODO

### Stage `IntermediateHashes`
TODO

### Stage `HashState`
TODO

### Indexing stage
There are 4 indexes that are generated during sync.
They might be disabled because they aren't used for all the APIs.
These stages do not use a network connection.

### Stage `AccountHistoryIndex`
TODO: refine this description

This index stores the mapping from the account address to the list of blocks where this account was changed in some way.

### Stage `StorageHistoryIndex`
TODO: refine this description

This index stores the mapping from the storage item address to the list of blocks where this storage item was changed in some way.

### Stage `LogIndex`
TODO

### Stage `CallTraces`
TODO

### Stage `TxLookup`
TODO: refine this description
This index sets up a link from the transaction hash to the block number.

### Stage `TxPool`
TODO: refine this description

During this stage we start the transaction pool or update its state. For instance, we remove the transactions from the blocks we have downloaded from the pool.

On unwinds, we add the transactions from the blocks we unwind, back to the pool.

This stage doesn't use a network connection.

### Stage `Finish`

This stage sets the current block number that is then used by [RPC calls](../../cmd/rpcdaemon/Readme.md), such as [`eth_blockNumber`](../../README.md).