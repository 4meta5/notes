# Substrate
> notes on [Substrate](https://github.com/paritytech/substrate)

Substrate consists of three main technologies: WebAssembly, Libp2p, and AfG Consensus. At a high level, it's a framework for creating decentralized systems. It abstracts away the necessary cryptographic primitives, networking code, and database storage in order to allow developers to focus on the decentralized system's state transition function.

> [What is Substrate?](https://www.parity.io/what-is-substrate/)

Substrate maintains a dynamic, self-defining state transition function. This STF is coded in WASM and is referred to as the "runtime". This defines the ```execute_block``` function while also specifying 
* staking algorithm
* transaction semantics
* logging mechanisms
* governance

Runtime is dynamic => any of these parts can be switched out!

## Light Client

Basic header format:
* `parent_hash`
* `extrinsics_root`
* `receipt_hash` (receipts form the basis of Substrate's approach to light clients)
    * `number`
    * `storage_root`
    * `changelog_root` -- represents the root node of the Merkle trie formed as the ordered storage keys that changed this block, themselves keyed with a sequential integer (enables light clients to receive trustless proofs that any key in storage didn't change or, if it did, the extrinsic(s) which changed it)
    * `digest` -- an array of fragments that the runtime can use to encode important events (enables trustless tracking of validator set changes to allow for *warp* sync)