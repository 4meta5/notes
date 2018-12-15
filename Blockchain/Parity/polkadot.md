# Polkadot
> random notes on polkadot; organized haphazards and **will be made into its own directory soon**

> [Polkadot Metalink](https://github.com/w3f/Web3-wiki/wiki/Polkadot)

> blockchain for scalability and interoperability

* [Generality](#generality)
* [Interperability](#interop)
* [Upgradeability](#upgrade)
* [Protocol](#protocol)

## Generality <a name = "generality" ></a>

We don't want to lock developers (or users) into any specific constraints wrt how data is stored, or metering for computation. The courser the grains of computation you meter for with fees, the more work you can do. 

### De-Duplication of Work

* comes with first-class light client

Substrate maintains a dynamic, self-defining state transition function. This STF is coded in WASM and is referred to as the "runtime". This defines the ```execute_block``` function while also specifying 
* staking algorithm
* transaction semantics
* logging mechanisms
* governance

Runtime is dynamic => any of these parts can be switched out!

## Interoperability <a name = "interop" ></a>

* Chains are initially isolated, and serve distinct purposes
* A messaging framework is overlayed
    * Blockchains can issue an event that leads to a transaction being committed to another chain; this is done in a deterministic and predictable way. Guaranteed arrival is another property that we strive to ensure for the messaging framework.
* Messages have predictable properties: in-order and eventual receipt

The **relay chain** negotiates the passing of messages between many other chains. It unites them all under a single consensus process; it also unites this hidden state of their message queues under this consensus process.

* Polkadot: network with a root relay chain which manages advancement and security of various parachains that have their own state machine and can communicate amongst eachother

> This is also a useful tool for scalability. This solution to interoperability plays well with scalability -- we can get things like hierarchical chains and form of sharding, by uniting distinct and heterogeneous state transitions under the same consensus process.

**Parachain Building Blocks**
* Validity Function: WebAssembly stored on-chain in the parachain registry
* Collator Node: Creates "candidate" blocks that satisfy the validity function
* Message Queues: candidates must also process incoming and produce outgoing messages

### Pooled Security

* change mining/validating from a zero-sum game to something more collaborative

### Scalability
At the root level, you run into quadratic scaling issues. If you have a bunch of chains sending messages amongst eachother, communication complexity scales quadratically; a fundamental scaling issues.

By localizing chains close to eachother to the ones they need to communicate with the most, we can reduce the scaling barrier. We can compress many chains communicating into one chain communicating with others. That sense of *locality* is very useful when overcoming fundamental scaling limits.

**Hierarchical Architecture**
* parachain which is also a relay chain -- 2nd order relay chain
    * trie-like structure of blockchains
    * security flows from the root

> consider trying to quantify how security lessens at each level (would be useful for ethereum l2 solutions as well)

### Finality

* Finality can always be reverted for a price
    * absolute finality really just means we have reached the price threshold (and that's what consensus algorithms try to do -- bring that price threshold to revert a block up to the maximum cost)

The problem with just creating a bridge protocol between chains is the increased ease by which we can attack weaker chains. Therefore, we need to make attaching one chain as expensive as attacking another chain.

### Locality
> DApps are likely to have a lot of different moving parts. Keeping portions of the on-chain application logic localized near or on the chains it interacts with most.

There is a more expensive cost to sending messages between chains. 

> embed modules for smart contracts into specific-purpose chains so you can essentially have proxy contracts that store information as close as possible to the module that it interacts with often

* asynchrony is a more powerful primitive, but there is some developer overhead
    * timeouts

* app devs would want to minimize the number of asynchronous calls in favor of using synchronous calls within chains
    * prefer doing synchronous work internally and farms out using asynchronous calls to do external work in the background
    * asynchronous calls only happen for large interchain transfers of value

* Smart contract platforms (like Ethereum) specialized for intra-operability. You can do communication within a smart contract platform synchronously and, therefore, very efficiently.
    * Optimized for maximizing the network effect within a specific bubble

## Upgradeability <a name = "upgrade" ></a>

* blockchains are encoded as a representation of its state transition function; registered on-chain on the root level relay chain
    * chain could issue a specific message for upgrading code
    * could also (simultaneously) register a new chain with new code (hard fork without hard forking) -- part of the community migrates over to the new chain
        * don't get into schism situation where we lose the network effects due to disagreement

> is it as easy to update the network layer? Look into this (ie switching between libp2p vs devp2p)

### No Forks?
> expand on above

### Governance

* adaptive quorum biasing
* progressive consensus
    * consensus algorithm which achieves finality slowly (/gradually)
        * blockchain - property that every block references all of the blocks before it
            * economic attestaion to the validity of the new block

* stakeholder referendum
* approval voting
* council voting
* qualified abstention biasing (<=> adaptive quorum biasing?)

## Protocol <a name = "protocol">

> [consensus](./consensus.md)

### Roles

**Validators**
* manage relay-chain block authorship
* parachain candidate agreement shuffled over parachains
    * collator gives parachain candidate block
    * validator approves/denies candidate block
* steward availability of external data
    * proof of custody?

**Nominators**
* stake on behalf of good validators
    * mitigate tradeoff between economic security and decentralization
* economic security without additional consensus overhead
* heuristic-based assignment

**Collators**
* create parachain candidates to give to validators
* work on single parachain only
* monitor parachain sub-net for misbehavior

**Fishermen**
* final line of defense: watch for misbehavior of validators
* anyone can be a fisherman
* can trigger the "validity/availability" game and slash bad validators

### Ensuring Availability

* checking parachain blocks requires off-chain data
    * guaranteeing data availability is nontrivial

## References and Resources

**Videos**
* [Implications of Interoperability at Web3 Summit 2018](https://www.youtube.com/watch?v=TBeGIGvC6r8&feature=youtu.be) by Rob Habermeier
* [Rob Habermeier presents Parity Substrate: the foundation for blockchain innovators](https://www.youtube.com/watch?v=q1zLHO7Lkuk)