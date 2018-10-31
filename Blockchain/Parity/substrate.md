# Substrate

> notes on [substrate](https://github.com/paritytech/substrate) codebase

Substrate consists of three main technologies: WebAssembly, Libp2p, and AfG Consensus.

Substrate maintains a dynamic, self-defining state transition function. This STF is coded in WASM and is referred to as the "runtime". This defines the ```execute_block``` function while also specifying 
* staking algorithm
* transaction semantics
* logging mechanisms
* governance

Runtime is dynamic => any of these parts can be switched out!