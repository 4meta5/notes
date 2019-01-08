# Accumulators

## ReadingQ

* [A Deep Dive on RSA Accumulators](https://beta.cent.co/+vcq48c)

> from Benedict Buntz on his Zcash grant project

[Batching techniques for RSA Accumulators and Vector Commitments](https://eprint.iacr.org/2018/1188)
* The paper presents new batching techniques for RSA accumulators and vector commitments. An **accumulator** is a short commitment to a set that supports efficient inclusion and optionally also exclusion proofs. 
* The perhaps simplest accumulator is a Merkle Tree which is widely used in ZCash for example as a commitment to all of the coins. 
* RSA accumulators have the advantage that their inclusion proofs are only a single element. In our work we show how many inclusion proofs can be non-interactively aggregated. 
* Additionally we leverage some recent work on [verifiable delay functions](https://eprint.iacr.org/2018/623) to make checking inclusion proofs much more efficient.
* We also leverage the new batching techniques to build an efficient vector commitment scheme. Vector commitments are positional commitments that allow you to efficiently prove that the element at the ith index has a certain value. Again Merkle Trees can be used as vector commitments. Our new commitment, however, is much more efficient in terms of proof size and we propose using it to create significantly shorter STARKs. To maintain the setup-freeness of STARKs one would have to use so called class groups to instantiate the accumulator (which is entirely possible and feasible).
There are more tricks in the paper and you can watch [this talk](https://www.youtube.com/watch?v=IMzLa9B1_3E&feature=youtu.be&t=3515) if you want to get a better overview.

Accumulators for ZCash:
There are several places where accumulators could be useful for ZCash. One interesting application is to make mining/full nodes stateless. Consider the t-address/Bitcoin like part of ZCash. An accumulator can be used to commit to the entire UTXO set. Each transaction now creates a short proof that her coins are indeed unspent. With our new techniques these proofs can be aggregated and are efficient to check. The miner only needs the short accumulator value (~256 bytes) in order to verify transactions! It might be unrealistic that users store their own inclusion proofs and update them regularly but so called bridge nodes could provide them for users. In general this design achieves a separation of consensus and state. The miners don?t need to store the entire state to reach consensus on it. A small commitment to the consensus suffices! Additionally it reduces the cost of ?UTXO dust? as it does not need to be stored in memory anymore.
Interestingly this design idea isn?t new and ZCash?s z-address system already uses such a design (using Merkle Trees + SNARKs instead of accumulators). Also ZeroCoin, ZeroCash?s predecessor, heavily relies on accumulators. An obvious question that we are investigating is whether and how efficient accumulators can be combined with SNARKs or other proof systems like Bulletproofs.