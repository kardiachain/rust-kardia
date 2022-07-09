# net package (or p2p)
It contains:
- p2p implementation on exchanging data (votes, proposal, blocks...) between peers

Its functionalities could be described as follow:

# p2p/node.rs
## Node.start_sync()
It spawns a following tasks:
1. [Sync Task] listens from `stream := Node.sync_stream()`
    - process `InbouldMessage`, they are `NewBlockHashes, BlockHeaders, NewBlock`
    - where msgs come from? -> obviously from its name `sync`, they come from staged_sync.
    - 
    - loop stream.next()
      - switch case on message type -> invoke corresponding handler
2. 