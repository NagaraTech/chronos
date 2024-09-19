# Core Functional Crates

The crates folder of Chronos includes core functional code crates and utility libraries, etc.

## [vlc](./vlc/)

- This verifiable logical clock crate implements a verifiable logical clock construct. 
- The clock can be used in a peer-to-peer network to order events. 
- Any node in the network can verify the correctness of the clock.

## [accumulator](./accumulator/)

- A simple accumulator application.
- Each accumulator node maintains a set of strings. Upon receiving a string from a client, the node adds the string to its state, and broadcast the new state to other nodes in the network. 
- All nodes eventually converge to the same state, by merging received states into their own states.

## [cops](./cops/)

- A causally consistent data store inspired by [COPS](https://www.cs.cmu.edu/~dga/papers/cops-sosp2011.pdf).
- The data store maintains a set of key-value pairs. 
- It provides causal consistency to clients.
