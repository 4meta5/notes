# Client
> *Our Goal is to create a protocol which is compatible with varying degrees of 'lightness,' from clients which store almost nothing to those which store almost everything.* - [The Parity Light Protocol](https://wiki.parity.io/The-Parity-Light-Protocol-%28PIP%29)

* [What is a light client and why you should care?](https://www.parity.io/what-is-a-light-client/)

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

* [Parity 1.2.2](https://www.parity.io/parity-1-2-2-released-hard-fork-enabled/) -- July 19, 2016
> "*Parity is a rust implementation of the Ethereum yellow paper that has been redesigned from the ground up to offer improved performance in day to day syncing and a lower resource footprint on your machine while running.*" This version included modification for the hard fork, improved syncing performance, and a few other bug fixes.

* [Parity 1.3](https://www.parity.io/announcing-parity-1-3/) -- August 17, 2016
> The parts of the clients were refactored for optimization: internal caching, database schemas, SHA3 batching and parallelization, VM gas accounting and data compression. This release reduced the state database by ~30% via domain-sepcific RLP compression (thereby reducing the disk footprint). It also increased modularization by improving the hypervisor architecture such that the network synchronization module of Parity can run entirely in its own process protecting it from compromising the rest of the core.

* [Parity 1.4](https://www.parity.io/announcing-parity-1-4/) -- November 07, 2016
> This release added Warp Sync, a highly optimized chain synchronization mode that uses various methods of compression to distribute the state of Ethereum. With strong internet connection, warp sync enabled synchronization in ~10 minutes (which is crazy fast!). This implementation included auto-throttling functionality to recognize transactions which degrade the network, throttle them, and allow others to take their place. This new feature protects against resource intensive attacks like [the one](https://www.parity.io/onwards/) experienced two months earlier.

* [Parity 1.5](https://www.parity.io/onwards/) -- January 25, 2017
> SO MUCH STUFF! The Parity wallet had been improved via multisig capability. Moreover, the UI was improved to enable better views of pending transactions as well as transaction history. The Parity Identity platform enabled the ability to associate accounts with particular badges or non-transferrable cryptographic certificates. In addition, two Proof-of-Authority consensus engines (Authority Round and Tendermint) were released. Lastly, this release included automatic updating -- the first fully blockchain-based consensus-protocol updating system. Updating decentralized systems is really hard and this release provides a way for Parity to seamlessly upgrade itself to a new release in situ, either automatically or via a button in Parity Wallet. 

* [Parity 1.6](https://www.parity.io/announcing-parity-1-6/) -- March 13, 2017
> This update provides new functionalities for the multisig including operability with hardware wallets (Ledger specifically), in-built Stratum protocol support for managing miner workloads, and an improved transaction queue process (which was updated to be persistent between restarts to prevent losing transactions). In addition, this release introduced Vaults to store all the metadata - address, name, and description - encrypted. In this way, vaults essentially represent groups of wallets that store encrypted metadata.

* [Parity 1.7](https://www.parity.io/announcing-parity-1-7/) -- July 28, 2017
> This update included experimental light client support. The team cited that the release is a preview for custom use-cases involving IoT and mobile, which is quite interesting because interaction with the blockchain has thus far been confined to desktop computers and advanced hardware. In addition, the team added authority contracts to the implementation of PoA chains. The contracts enable defining authorities in entirely arbitrary way thereby opening the door for derivative consensus systems like PoS in the future.

Interestingly, 1.7 also introduced WebAssembly as an alternative runtime to the EVM for smart contract execution. When more improvements are made on the WASM engine, this will open up the languages for programming smart contracts to C, Typescript, Hax, and **Rust**.

* [Parity 1.8](https://www.parity.io/announcing-parity-1-8/) -- October 15, 2017
> This release enables official support for the Trezor hardware wallet. Moreover, it added dynamic authority sets (important for permissionless consensus) to Proof of Authority chains as well as feature compatibility with the Whisper v6 wire protocol. In addition, 1.8 entailed a major refactor of Parity's internals in order to decouple consensus logic from transaction and block verification logic. This allows for increased flexibility when implementing pluggable consensus as well as pluggable state transition systems. 

1.8 also represented the launch of the Parity Mobile Signer for iOS and Android. The application allows you to introduce an air gap between nodes and private keys whereby the keys are stored securely on a mobile device while transactions are sent to the Parity Wallet Signer on a node. 

* [Parity 1.9](https://www.parity.io/velocity-the-fastest-parity-released/) -- January 26, 2018
> This release provided upgrades to the underlying database, RocksDB, to make it more stable in the context of initial configuration. At the network layer, the updated protocol utilized devp2p alongside Snappy compression, thereby using approximately 50% less network traffic. The new release also improved the performance and stability of Aura (Proof of Authority) networks, and added a number of tools for key management.

* [Parity-Ethereum 2.0.0](https://www.parity.io/parity-ethereum-2-0/) -- July 18, 2018
> This release placed the Parity Ethereum blockchain client in the context of a growing stack of web3 technologies that would be built over the course of 2018 and 2019. Moreover, the update comes with a highly customizable chain specification thereby allowing pluggable consensus engines. Moreover, it allows users to choose between the EVM and WasmVM. 

Because Parity Ethereum is built to serve as expert software for production use and shouldn't be considered for end-user software, Parity decided to remove the Parity Wallet from the client as well as all installers and OS-specific packages. The upcoming wallet alternative will be incorporated in [Parity Fether](https://github.com/paritytech/fether) light wallet. 