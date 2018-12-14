# The Data Availability Problem

* The Statement of the Data Availability Problem -- [here's an intro](https://github.com/ethereum/research/wiki/A-note-on-data-availability-and-erasure-coding)
* Speaker/Listener Fault Equivalence

As Ethereum transitions to a PoS sidechain and other interoperability-focused networks emerge (i.e. Polkadot and Cosmos), the data availability problem will take on increased significance. With this in mind, we need to reintroduce a solution for speaker/listener fault equivalence that was identified as early as the canonical paper: [The Byzantine Generals Problem](https://people.eecs.berkeley.edu/~luca/cs174/byzantine.pdf) by Lamport, Shostak, and Pease.

We'll start by reviewing a blog post released by Vitalik Buterin on this topic in August 2018. Then, we'll review the same idea explored by Gavin Wood in the original Polkadot whitepaper two years earlier.

## Increasing Fault Tolerance via Observers
> [A Guide to 99% Fault Tolerant Consensus](https://vitalik.ca/general/2018/08/07/99_fault_tolerant.html) by Vitalik Buterin

If we add observers that actively watch consensus, we can increase fault tolerance all the way to 99%.

If the assumptions of both the threshold-dependent and latency-dependent consensus algorithms are broken *at the same time* (or in consecutive rounds), then the algorithm can break down...However, this is unavoidable; the impossibility of safe-under-asynchrony consensus is more than 1/3 fault tolerance is a well-established result in Byzantine Fault Tolerance Theory, as is the impossibility of more than 1/2 fault tolernace even allowing synchrony assumptions but assuming offline observers.

## Polkadot Solution
> Section 6.5.3 (page 13) of the [Polkadot Whitepaper](https://polkadot.network/PolkaDotPaper.pdf)

**Mitigating the Data Availability Problem**<br>
1. *Public Participants*, similar to fishermen, police the validators who claim availability; specifically, their task is to find the validators that appear unable to demonstrate validity and lodging corresponding micro-complaints to other validators. **Speaker/Listener Fault Equivalence still exists here; we just increase complexity!**
2. *Availability Guarantors*: nominate a second set of bonded validators that attest to the availability of all important interchain data. 
> This has the advantage of relaxing the equivalence between *participants* and *chains*. Essentially chains can grow (along with the original chain validator set), whereas the participants, and specifically those taking part in data availability testament, can remain at the least sub-linear and quite possibly constant.

3. Collators have an intrinsic incentive to ensure that all data is available for their chosen parachain since without it they are unable to author further blocks from which they can collect transaction fees. Recent collators are given the ability to issue challenges to the availability of external data for a particular parachain block to validators for a small bond...Validators must contact those from the apparently offending validator sub-groups who testified and either acquire and return the data to the collator or escalate the matter by testifying the lack of availability (direct refusal to provide the data counts as a bond-confiscating offense, therefore the misbehaving validator will likely just drop the connection) and contacting additional validator to return the same test. In the latter case, the collator's bond is returned...once a quorum of validators who can make such non-availability testimonials is reached, they are released, the misbehaving sub-group is punished, and the block is reverted.

## Proof of Custody

* [Enforcing windback (validity and availability), and a proof of custody](https://github.com/ethereum/research/wiki/A-note-on-data-availability-and-erasure-coding)
* [Finality and Windback - Proof of Custody Revisited](https://ethresear.ch/t/finality-and-windback-proof-of-custody-revisited/1434)
* [Extending skin-in-the-game of notarization with proofs of custody](https://ethresear.ch/t/extending-skin-in-the-game-of-notarization-with-proofs-of-custody/1639)

### References
* [A note on data availability and erasure coding](https://github.com/ethereum/research/wiki/A-note-on-data-availability-and-erasure-coding)
* [Unsolved Problems in Blockchain Sharding](https://medium.com/nearprotocol/unsolved-problems-in-blockchain-sharding-2327d6517f43)