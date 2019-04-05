# Reading List
> papers that I want to read that I haven't read yet

* [The Writings of Leslie Lamport (abridged)](https://blog.bigchaindb.com/the-writings-of-leslie-lamport-abridged-a67df77f464)
* [The Writings of David Chaum (abridged)](https://medium.com/@mikerahqc/the-writings-of-david-chaum-abridged-c1c885b2bb64)

* [Batching Techniques for Accumulators with Applications to IOPs and Stateless Blockchains](https://eprint.iacr.org/2018/1188) -- Boneh, Bunz, and Fisch (the Stanford trio), December 11, 2018

* [Confidential Transactions have arrived, a dive into the AZTEC Protocol](https://medium.com/aztec-protocol/confidential-transactions-have-arrived-a-dive-into-the-aztec-protocol-a1794c00c009)

* [Compact Multi-Signatures for Smaller Blockchains](https://eprint.iacr.org/2018/483.pdf) - Boneh 2018
* [Signatures with Flexible Private Key: Introducing Equivalence Classes for Public Keys](https://eprint.iacr.org/2018/191.pdf) -- 2018
* [On Multiparty Garbling of Arithmetic Circuits](https://eprint.iacr.org/2017/1186.pdf) -- 2017

* [Improving Authenticated Dynamic Dictionaries, with Applications to Cryptocurrencies](https://eprint.iacr.org/2016/994.pdf) -- 2017

* [Trail of Bits: Guide to Post-Quantum Cryptography](https://blog.trailofbits.com/2018/10/22/a-guide-to-post-quantum-cryptography/)

* [On the Security and Performance of Proof of Work Blockchains](https://eprint.iacr.org/2016/555.pdf)

* [Adversaries, Distributed Ledgers & Decentralization](https://fieldnotes.resistant.tech/dags-and-decentralization/)

* [Dalek Bulletproofs Rust Crate](https://doc-internal.dalek.rs/bulletproofs/)
* [Programmable Constraint Systems for Bulletproofs](https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7)

* [Zexe: Enabling Decentralized Private Computation](https://eprint.iacr.org/2018/962.pdf)

* [A CCA-secure collision-resistant Identity-based Proxy Re-Encryption Scheme](https://eprint.iacr.org/2018/1131.pdf)

* [Selected Papers in Anonymity (metalink)](https://www.freehaven.net/anonbib/)

## Random Notes on Computation Bounds

Reading [Establishing Root of Trust Unconditonally](https://blog.acolyer.org/2019/04/03/establishing-software-root-of-trust-unconditionally/) and I want to cache a few property definitions.

**What does unconditional imply about the security assumptions?**
* The verifier asks the system to initialise its random access memory M and register set R to the chosen content (i.e., known good state of a trustworthy program).
* Then the verifier selects a random nonce and sends it to the system with a challenge to perform a (very!) carefully chosen computation over M and R and the nonce.
* The computation is such that it is **space-time optimal** (i.e., not possible to do it any faster than time t, and not possible to use any less space than m).
* The computation is **non-interruptible**, and is timed with very high accuracy. The verifier expects a result after time t, such that there isn’t time to slip in even a single additional instruction.
* Moreover, the computation is **second pre-image free**. This means that you can’t easily find a second input (e.g., a somehow corrupted version of the desired system state in this case) that produces the same output as the original input (the system state we expect the system to be in).

The lower bounds of a computation are established by a proof that holds for all possible algorithms for performing it. For example, if we’re going to add two numbers we at least need to look at those two numbers… A particular algorithm for that computation is space-time optimal if its bounds match the space and time lower bounds of its computation. Asymptotic lower bounds are no use here (because an adversary might always be able to find slightly better concrete bounds), and nor are purely theoretical lower bounds: we must have a concrete algorithm that exactly matches them. Moreover, the algorithm needs to be executable in a realistic computer model (e.g. , accounting for general ISAs, I/Os, interrupts, multi-processors, and so on).

**[Horner's Rule](http://mathworld.wolfram.com/HornersRule.html)**