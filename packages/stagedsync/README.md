# stagedsync
This package gets inspired by StagedSync in Erigon architecture. Read more:
https://github.com/ledgerwatch/erigon/blob/devel/eth/stagedsync/README.md

Both `erigon` (Go) and `akula` (Rust) implement `stagedsync`. Please refer to them for further reading.

## How staged sync work?
For each peer Erigon learns what the HEAD blocks is and it executes each stage in order for the missing blocks between the local HEAD block and the peer's head blocks.
The first stage (downloading headers) sets the local HEAD block.
Each stage is executed in order and a stage N does not stop until the local head is reached for it.
That mean, that in the ideal scenario (no network interruptions, the app isn't restarted, etc), for the full initial sync, each stage will be executed exactly once.
After the last stage is finished, the process starts from the beginning, by looking for the new headers to download.
If the app is restarted in between stages, it restarts from the first stage.
If the app is restarted in the middle of the stage execution, it restarts from that stage, giving it the opportunity to complete.

## Questions and considerations
- Should we implement `stagedsync` in `rust-kardia`?
- How validator and staking implemented with `stagedsync`?