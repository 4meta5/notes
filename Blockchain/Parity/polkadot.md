# Polkadot
> random notes on polkadot; organized haphazards and **will be made into its own directory soon**

> [Polkadot Metalink](https://github.com/w3f/Web3-wiki/wiki/Polkadot)

## Generality

We don't want to lock developers (or users) into any specific constraints wrt how data is stored, or metering for computation. The courser the grains of computation you meter for with fees, the more work you can do. 

## Interoperability

* Chains are initially isolated, and serve distinct purposes
* A messaging framework is overlayed
    * Blockchains can issue an event that leads to a transaction being committed to another chain; this is done in a deterministic and predictable way. Guaranteed arrival is another property that we strive to ensure for the messaging framework.
* Messages have predictable properties: in-order and eventual receipt

The **relay chain** negotiates the passing of messages between many other chains. It unites them all under a single consensus process; it also unites this hidden state of their message queues under this consensus process.

> This is also a useful tool for scalability. This solution to interoperability plays well with scalability -- we can get things like hierarchical chains and form of sharding, by uniting distinct and heterogeneous state transitions under the same consensus process.

### Scalability
At the root level, you run into quadratic scaling issues. If you have a bunch of chains sending messages amongst eachother, communication complexity scales quadratically; a fundamental scaling issues.

By localizing chains close to eachother to the ones they need to communicate with the most, we can reduce the scaling barrier. We can compress many chains communicating into one chain communicating with others. That sense of *locality* is very useful when overcoming fundamental scaling limits.

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

## Upgradeability

* blockchains are encoded as a representation of its state transition function; registered on-chain on the root level relay chain
    * chain could issue a specific message for upgrading code
    * could also (simultaneously) register a new chain with new code (hard fork without hard forking) -- part of the community migrates over to the new chain
        * don't get into schism situation where we lose the network effects due to disagreement

### No Forks?
* this is nuanced; expand

### References and Resources

**Videos**
* [Implications of Interoperability at Web3 Summit 2018](https://www.youtube.com/watch?v=TBeGIGvC6r8&feature=youtu.be) by Rob Habermeier