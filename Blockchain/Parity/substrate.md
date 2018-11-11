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