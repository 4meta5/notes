# Client

Parity gained critical experience building Ethereum clients prior to pursuing the Substrate and Polkadot projects. Here we will provide a rough timeline based on blog posts on the Parity [website](https://parity.io).

* [Performance Analysis](https://www.parity.io/performance-analysis/) -- February 2, 2016
> Parity develops and releases the fastest and lightest fully compliant Ethereum block processing engine available.

* [Parity 1.0](https://www.parity.io/parity-1-0-is-here/) -- March 24, 2016
> This added (1) Ethereum JSONRPC implementation, (2) zero-configuration compatibility with ethminer for easy solo mining, (3) full compatibility with geth over key files and CLI options. In this release, Parity also added state-trie pruning support (to reduce overall burden on disk), chain-specification file support (to easily configure and deploy private chains), and hierarchical mip-mapped log-blooming (for near-instantaneous log querying).

* [Parity 1.1](https://www.parity.io/announcing-parity-1-1/) -- May 2, 2016
> Codenamed Alacrity (my favorite name of any of the releases!), this version provided mostly "under-the-hood" improvements and optimizations including but not limited to Ubuntu 16.04 support, substantial refactoring of the JSON codec, and optimized mining code for keeping track of nonces affiliated with transaction data.

* [Parity 1.2](https://www.parity.io/announcing-parity-1-2/) -- June 24, 2016
> The major addition for this milestone included Windows build, IRC/RPC support, Signing UI, Dapp-hosting, Optimized mining support, Transaction-tracing support, and DAO soft-fork support. The Signing UI provided a fundamentally secure means of managing all transaction signing for secret keys. The key innovation behind this release was a rotating authentication token, which is used to ensure that no other process can silently steal the privileges of the Signing UI.

One decision worthy of discussion was whether or not to make the default user choice to either support or not support the DAO rescue via soft fork. In the 1.2 release, Parity cited that "supporting the DAO-rescue soft-fork attempt requires no additional work. If you do not wish to support the soft-fork, run with ```--dont-assist-dao-rescue```." This decision to make the default user choice supportive of the soft fork makes sense in the context of [Parity's response](https://www.parity.io/our-dao-response/), but it is by no means *neutral*. I guess the important thing to consider is that **there is no neutral choice**; Parity's decision in this instance reflects their views on the appropriate response to the DAO hack.

