
* [protocol development ideas](#protocol)
* [Use Cases](#use)

# Protocol Development Ideas <a name="protocol"></a>

* [Radical Consensus Substrate Blockchain](#RadicalConsensus)
* [Optimizing SimpleSerialize](#ssz)

## Radical Consensus Substrate Blockchain <a name="RadicalConsensus"></a>

I want to hack on the [Shasper beacon chain implementation using the Substrate framework](https://github.com/paritytech/shasper). I have some ambitious goals on this project to integrate Radical Markets ideas into the consenus mechanism itself.

1. constant/continuous auction for validator voting spots (incorporate a delay to prevent long-range attacks)
2. Quadratic Voting to weight validator votes based on staked capital

## Optimizing SimpleSerialize <a name="ssz"></a>

Vitalik came up with the [spec](https://github.com/ethereum/eth2.0-specs/blob/master/specs/simple-serialize.md) for serializing and deserializing objects and data types for Ethereum 2.0 using the Beacon Chain.

> I have a hunch that this spec is not optimized and can be improved upon. Check out [ssz implementation by sigp/lighthouse](https://github.com/sigp/lighthouse/tree/master/beacon_chain/utils/ssz).

> Need to understand this in-depth and consider interaction with WebAssembly...can we further optimize this with the expectation that we are interacting with WebAssembly on the front-end?

# Use Case Ideas <a name="use"></a>

* [University](#uni)

## University <a name="uni"></a>

By increasing accessibility to learning resources, the internet has challenged the role of the university as the primary gatekeeper for knowledge (or at least higher level education). 

I think the blockchain (or more generally, distributed systems and cryptography) will enable ease of tracking verifiable digital credentials, thereby enabling *autodidacts*, or students of the internet, to prove their knowledge. This **should** be the final blow to take down the monopology of universities over higher level education, but here we ignore the cultural and psychological attachment to college. Parents want their kids to do it because they did it. People also don't think about how the world changes, only about the status quo.