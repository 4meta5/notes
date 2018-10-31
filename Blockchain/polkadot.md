# Notes on Polkadot

[Paper](https://github.com/polkadot-io/polkadotpaper/raw/master/PolkaDotPaper.pdf)

* [History](#history)
* [Properties](#properties)
    * [Roles](#roles)
    * [Interchain Communication](#communication)
* [Protocol](#protocol)
    * [Stake-token liquidity](#liquidity)

**Why existing protocols maintain wide timing margins on the expected processing time?**<br>
The state transition mechanism, or the means by which parties collate and execute transactions, has its logic fundamentally tied into the consensus “canonicalisation" mechanism, or the means by which parties agree upon one of a number of possible, valid, histories.

*Polkadot <=> decouple consensus architecture from state transition mechanism*

## History <a name = "history"></a>

A more complex scalable solution known as Chain fibers, dating back to June 2014 and first published later that year, made the case for a single relay-chain and mul- tiple homogeneous chains providing a transparent inter- chain execution mechanism. Decoherence was paid for through transaction latency—transactions requiring the coordination of disparate portions of the system would take longer to process. [source](https://github.com/ethereum/wiki/wiki/Chain-Fibers-Redux)

> high-frequency chains (with very low block times)

## Properties <a name = "prooperties"></a>

Polkadot is equivalent to a set of independent chains except for two important points:
1. Pooled security
2. Trust-free transaction transactability

Mark Twain:
> “Governments and diapers must be changed often, and for the same reason”

### Roles <a name = "roles"></a>

**Validator**:<br>
* must run a relay-chain client implementa- tion with high availability and bandwidth
    *  the node must be ready to accept the role of ratifying a new block on a nominated parachain
        * ratifying => receiving, validating and republishing candidate blocks
* Once all new parachain blocks have been properly ratified by their appointed validator subgroups (**collators**), validators must then ratify the relay-chain block itself. T
    * To ratify the relay-chain block: updating the state of the transaction queues (essentially moving data from a parachain’s output queue to another parachain’s input queue), processing the transactions of the ratified relay-chain transaction set and ratifying the final block, including the final parachain changes.
    
**Nominator**: <br>
* A nominator is a stake-holding party who contributes to the security bond of a validator.

**Collators**:<br>
* maintain a “full-node” for a par- ticular parachain
* collate and execute transactions to create an unsealed block, and provide it, together with a zero-knowledge proof, to one or more validators presently responsible for proposing a parachain block

> collator pools who vie to collect the most transaction fees

> decentralised nominator pools would allow multiple bonded participants to coordinate and share the duty of a validator

**Fishermen**:<br>
* get their reward through a timely proof that at least one bonded party acted illegally
    * Illegal actions include signing two blocks each with the same ratified par- ent or, in the case of parachains, helping ratify an invalid block
* fishermen must post a small bond (to prevent sybil attacks from wasting validator compute resource)
    * they can achieve a hefty profit from identifying bad behavior (but this will occur relatively infrequently)

**???**<br>
Polkadot, however, also provides strong guarantees that the parachains’ state transitions are valid. This happens through the set of validators being cryptographically randomly segmented into subsets; one subset per parachain, the subsets potentially differing per block. This setup generally implies that parachains’ block times will be at least as long as that of the relay-chain. 
> WHY?

> a Bitcoin-like chain which has a much simpler fee model or some other, yet-to-be-proposed spam-prevention model

### Interchain Communication <a name="communication"></a>

* transactions executing in a parachain are (according to the logic of that chain) able to effect the dispatch of a transaction into a second parachain or, potentially, the relay-chain. 
    * Like external transactions on production blockchains, they are fully asynchronous and there is no intrinsic ability for them to return any kind of information back to its origin.

* Interchain transactions are resolved using a simple queuing mechanism based around a Merkle tree to ensure fidelity. It is the task of the relay-chain maintainers to move transactions on the output queue of one parachain into the input queue of the destination parachain.

* These queues are administered on the relay-chain allowing parachains to determine each other’s saturation status; this way a failed attempt to post a transaction to a stalled destination may be reported synchronously. 
**(Though since no return path exists, if a secondary transaction failed for that reason, it could not be reported back to the original caller and some other means of recovery would have to take place.)**

**Interoperability with Ethereum**<br>
We can imagine our Polkadot-side Ethereum-interface will have a few simple functions:
1. able to accept a new header from the Ethereum network and validate the PoW
2. able to accept some proof that a particular log was emitted by the Ethereum-side break-out contract for a header of sufficient depth (and forward the corresponding message within Polkadot)
3. able to accept proofs that a previously accepts but not-yet-enacted header contains an invalid receipt root

**What does the breakout contract look like?**

**How is this incentivization managed?**<br>
> To actually get the Ethereum header data itself (and any SPV proofs or validity/canonicality refutations), an incentivization for forwarding data is necessary.

## Protocol <a name="protocol"></a>
The relay-chain is state-based with the state mapping address to account information (mainly balances and (to prevent replays) a transaction counter).
> placing accounts here provides accounting for which identity possesses what amount of stake in the system

Notable differences from Ethereum:
* contracts cannot be deployed through transactions (to avoid application functionality on the relay-chain)
* compute resource usage ("gas") is not accounted; flat fee applies, thereby allowing for more performance from any dynamic code execution that may need to be done as well as a simpler transaction format
* special functionality supported for listed contracts that allows for auto-execution and network message outputs

### Stake-token liquidity <a name="liquidity"></a>

To tie the network security to the *market capitalization* of the staking token, as many tokens as possible should be staked within the network maintenance operations.
> we could incentivize this with a high interest rate paid out to validators

**Key problem**: if the token is locked in the Staking Contract under punishment of re- duction, how can a substantial portion remain sufficiently liquid in order to allow price discovery?

Potential solution? straight-forward derivative contract, securing fungible tokens on an underlying staked token...but this is difficult to create in a trust-free manner and there is an inherent disparity between the value of the derivative and the underlying.

Parity's solution: forcibly make 20% of token supply liquid

**targeting via reverse auction:** token holders interested in becoming a validator post an offer to the staking contract stating their minimum payout rate for participation. At the beginning of each session (maybe once an hour), validator slots are filled according to each would-be validator's stake and payout rate

> **who is working on this? have they considered a COST for validator slots...this would make more sense for what I expect to be relatively volatile returns (in fiat terms)**

> question: if we use inflation to incentivize staking, wouldn't the liquid token supply continuously decline in value? How is such a scheme sustainable?

**bond confiscation/burning -- will parameters be static or dynamic?**

**Sealing relay blocks** entails the collection of signed statements from validators over the validity, availability and canonicality of a particular relay-chain block and the parachain blocks that it represents.

The BFT consensus algorithm is motivated by Tangaora (a BFT variant of Raft), Tendermint, and HoneyBadgerBFT. The algorithm must reach an agreement on multiple parachains in parallel.

> page 11 stopped