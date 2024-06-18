# Demos

The demos folder of VLC includes code and document collections, including usage examples, design plans, integration, etc.

## [VLC-Dag](./vlc-dag/)

A vlc-dag application. The vlc-dag use case demo is base on the simple accumulator application. 

Please reference on [crates/accumulator](../crates/accumulator/). But add new features as follow:

* Defined the vertex and edge of the event & clock propagation dag.
* Support the LMDB to persistence storage for now，
* Maybe time-series db,like influxDB, or related postgre more suitable.
* Support increment state sync with peers using p2p communication protocol.

So we can use the vertex and edge from db to reconstruct clock propagation dag.

## [Randomness](./Randomness/)

This use case demo is mainly consist of two proposals.

#### VLC & BLS Proposal
Randomness serves a vital role in nearly every aspect of current society，the idea is to intergrate the ablility of logical clocks into random generator. To generate verifiable, fair random numbers, the proposal integrates BLS.

#### VLC & VRF Proposal
Randomness serves a vital role in nearly every aspect of current society，the idea is to intergrate the ablility of logical clocks into random generator. To generate verifiable, fair random numbers, the proposal integrates VRF.

## [Test-Conflict](./test_conflict/)

This use case domo is designed to detect software version conflict by applied vector clock.

## [Coll-Tx](./coll-tx/)

The coll-tx demo is about collecting simple utxo transaction with cops and vlc. 