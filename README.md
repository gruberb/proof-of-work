# Bitcoin Mining Coordination

This repostory is an example implemenation of different actors in a proof of work blockchain network.

The simulation deals with two kinds of actors: The `coordinator` and one or more `agents`.

## Coordinator
The `coordinator` is responsible for opening up a `TCP` `PORT`, and accepting connections from `agents`. It sends periodically the latest `BlockHeader` to all connected agents. Initially, it sends the genesis BlockHeader, with the `nonce` set to `0`.

## Agent
The `agents` job is it to calculate a so-called `nonce`. By finding a hash (which is calculated based on the block header) which satisfies the set difficulty of the network, it can increase the `nonce` by `1`, and send the `BlockHeader` back to the `coordinator`.

## Pre-requisites

- Rust `1.68` or newer

## How to run

1. Clone the repository (in this case, `cd` into the folder `bitcoin_mining_coordination`)
2. Run `cargo run`

## Example output

```
❯ cargo run
   Compiling bitcoin_mining_coordination v0.1.0 (FOLDER/bitcoin_mining_coordination)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/bitcoin_mining_coordination`
Spawn new agent
Spawn new agent
Spawn new agent
Spawn new agent
Spawn new agent
Connected to the coordinator: 127.0.0.1:3000
Connected to the coordinator: 127.0.0.1:3000
Connected to the coordinator: 127.0.0.1:3000
Connected to the coordinator: 127.0.0.1:3000
Connected to the coordinator: 127.0.0.1:3000
SENDING LATEST BLOCK HEADER
Broadcast new block BlockHeader { version: 4, prev_block_hash: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], merkle_root: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], time: 1702004049, difficulty: [0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], nonce: 0 }
SENDING LATEST BLOCK HEADER
SENDING LATEST BLOCK HEADER
SENDING LATEST BLOCK HEADER
Broadcast new block BlockHeader { version: 4, prev_block_hash: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], merkle_root: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], time: 1702004049, difficulty: [0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], nonce: 0 }
Broadcast new block BlockHeader { version: 4, prev_block_hash: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], merkle_root: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], time: 1702004049, difficulty: [0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], nonce: 0 }
Broadcast new block BlockHeader { version: 4, prev_block_hash: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], merkle_root: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], time: 1702004049, difficulty: [0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], nonce: 0 }
Broadcast new block BlockHeader { version: 4, prev_block_hash: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], merkle_root: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], time: 1702004049, difficulty: [0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], nonce: 0 }
SENDING LATEST BLOCK HEADER

...
```
