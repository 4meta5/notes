# Consensus
* [Proof of Work](./Basics/PoW.md)
* [Tradeoffs](./FunTradeoffs.md)

**Instant Finality**<br>
A non-probabilistic consensus protocol that gives a guarantee of finality immediately upon block production, for example Tendermint and Rhododendron. These tend to be PBFT based and thus very expensive in terms of communication requirements.
* Tendermint
* Rhododendron
* Stellar
* HoneyBadgerBFT
    * asynchronous "safety" and asynchronous "liveness" (if tweaked)

**Eventual Finality**
* Aura
    * Deterministic consensus protocol with Non-Instant Finality where block production is achieved by a rotating list of authorities that take turns issuing blocks over time, and where the majority of those online are honest authorities.
* Aurand
    * A variant of Aura where authorities are shuffled randomly on each round, increasing security.

**Probabilistic Finality**
* Aurand/Ouroboros
    * Extension of Aurand where block production involves validators competing over limited slots that move very quickly, where most slots are not populated, but with rare slot collisions.

**Hybrid Consensus**<br>
Split blockchain consensus into Block Production and a Finality Gadget. This allows chain growth speed to be as fast as in probabilistic "safety" consensus such as Ouroboros or Aurand but with same level of security guarantees as in Instant-Finality Consensus Protocols.
* Aurand+GRANDPA
    * A hybrid consensus scheme where Aurand is used for block production and short-term probabilistic-finality, with long-term absolute finality provided through GRANDPA.

**Gossip Protocol(s)**
* Avalanche

**Other**
* Shasper
* Casper FFG
* Consensus by bet

* [Holographic Consensus by DAO Stack](https://medium.com/daostack/holographic-consensus-part-1-116a73ba1e1c)

**Casper CBC**

## ReadingQ

* [FlyinFox Proof of Stake Blockchain](https://github.com/BumblebeeBat/FlyingFox)

* [Compounding Wealth in Proof-of-Stake Cryptocurrencies](https://arxiv.org/abs/1809.07468)
    * very interesting recent paper on practical reward parameterization

* [Using the Beacon Chain to POS-finalize the ETH 1.0 chain](https://ethresear.ch/t/using-the-beacon-chain-to-pos-finalize-the-ethereum-1-0-chain/4521)

* [What's New in Eth2](https://notes.ethereum.org/c/Sk8Zs--CQ/https%3A%2F%2Fbenjaminion.xyz%2Fnewineth2%2F20181210.html)

* [Snowflake to Avalanche](https://ipfs.io/ipfs/QmUy4jh5mGNZvLkjies1RWM4YuvJh5o2FYopNPVYwrRVGV)
* [PHANTOM. GHOSTDAG](https://eprint.iacr.org/2018/104.pdf) -- Two Scalable BlockDAG protocols

* [(Partially) Explained Casper CBC Specs](https://medium.com/@barnabe/partially-explained-casper-cbc-specs-86d055fd0628)
* [Casper CBC Simplified](https://medium.com/@aditya.asgaonkar/casper-cbc-simplified-2370922f9aa6)
* [Vitalik CBC Explainer](https://vitalik.ca/general/2018/12/05/cbc_casper.html)
* [Vitalik Part 2: Bitwise LMD Ghost](https://ethresear.ch/t/bitwise-lmd-ghost/4749)

* [Proof of Stake: How I Learned to Love Weak Subjectivity](https://blog.ethereum.org/2014/11/25/proof-stake-learned-love-weak-subjectivity/)

* [Ouroboros Crypsinous: Privacy-Preserving Proof-of-Stake](https://eprint.iacr.org/2018/1132)

**Against PoS**
* [PoS Require Subjectivity to Reach Consensus](https://forum.blockstack.org/t/pos-blockchains-require-subjectivity-to-reach-consensus/762?u=muneeb)
    * [Relevant Paper](https://eprint.iacr.org/2016/919.pdf)
* [Proof-of-Stake & the Wrong Engineering Mindset](https://medium.com/@hugonguyen/proof-of-stake-the-wrong-engineering-mindset-15e641ab65a2)

* [Alistair Stewart presents Consensus and Finality at Polkadot Seoul](https://www.youtube.com/watch?v=o1eKVi5ymSY)