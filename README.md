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

1. Clone the repository `git clone git@github.com:gruberb/proof-of-work.git`
2. `cd proof-of-work`
3. Run `cargo run`

## Example output

```
❯ cargo run
   Compiling bitcoin_mining_coordination v0.1.0 (bitcoin_mining_coordination)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/bitcoin_mining_coordination`
2023-12-15T17:18:14.185260Z  INFO bitcoin_mining_coordination::agent: Successfully connected to coordinator at 127.0.0.1:3000
2023-12-15T17:18:14.185225Z  INFO bitcoin_mining_coordination::agent: Successfully connected to coordinator at 127.0.0.1:3000
2023-12-15T17:18:14.185306Z  INFO bitcoin_mining_coordination::agent: Successfully connected to coordinator at 127.0.0.1:3000
2023-12-15T17:18:14.185294Z  INFO bitcoin_mining_coordination::agent: Successfully connected to coordinator at 127.0.0.1:3000
2023-12-15T17:18:14.185242Z  INFO bitcoin_mining_coordination::agent: Successfully connected to coordinator at 127.0.0.1:3000
2023-12-15T17:18:29.187063Z  INFO bitcoin_mining_coordination::agent: Received new block header from coordinator
2023-12-15T17:18:29.187128Z  INFO bitcoin_mining_coordination::agent: Received new block header from coordinator
2023-12-15T17:18:29.193771Z  INFO bitcoin_mining_coordination::agent: Received new block header from coordinator
2023-12-15T17:18:29.193888Z  INFO bitcoin_mining_coordination::agent: Received new block header from coordinator
2023-12-15T17:18:29.193888Z  INFO bitcoin_mining_coordination::agent: Received new block header from coordinator
2023-12-15T17:18:29.407316Z  INFO bitcoin_mining_coordination::coordinator: COORDINATOR: RECEIVED BLOCK: BlockHeader { version: 4, prev_block_hash: [211, 170, 137, 112, 132, 171, 248, 184, 88, 159, 143, 109, 84, 113, 50, 2, 146, 173, 70, 155, 43, 221, 246, 109, 165, 203, 167, 162, 159, 43, 132, 98], merkle_root: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], time: 1702660709, difficulty: [0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], nonce: 15096 }
2023-12-15T17:18:29.408568Z  INFO bitcoin_mining_coordination::coordinator: Block already received, ignoring
2023-12-15T17:18:29.409294Z  INFO bitcoin_mining_coordination::coordinator: Block already received, ignoring
2023-12-15T17:18:29.409494Z  INFO bitcoin_mining_coordination::coordinator: Block already received, ignoring
2023-12-15T17:18:29.410941Z  INFO bitcoin_mining_coordination::coordinator: Block already received, ignoring
...
```
