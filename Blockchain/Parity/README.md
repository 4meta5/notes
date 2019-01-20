# Parity
> [Parity](https://parity.io)

* [Praise for Parity](./<3.md)
* [Core](#core)
* [Substrate](#substrate)
* [Polkadot](#polkadot)

## Core <a name = "core"></a>

Parity has lots of experience building infrastructure to support blockchains.

* [Parity Bridge](./Core/bridge.md)
* [Parity Secret Store](./Core/secretstore.md)
* [Parity Ethereum](./Core/ethereum.md)
* [Parity Zcash (history and motivation)](./Core/zcash.md)
* [frontend](./Core/frontend.md)

## Substrate <a name = "substrate"></a>
> [What is Substrate?](https://www.parity.io/what-is-substrate/)  

* [Substrate](./Substrate/)
* [consensus](./Substrate/consensus.md)

## Polkadot  <a name = "polkadot"></a>
> [Polkadot Metalink](https://github.com/w3f/Web3-wiki/wiki/Polkadot)

* [Polkadot](./Polkadot/)
* [metagovernance (debate)](./Polkadot/metagovernance.md)
* [Parachain](./Polkadot/parachain.md)
* [Rationale](./Polkadot/rationale.md)
* [Whitepaper (notes)](./Polkadot/whitepaper.md)

## Miscellaneous

* [Althea Mesh Bidirectional Payment Channels](https://github.com/althea-mesh/guac_rs)

* using Bellman repo for Zero Knowledge constructions on Substrate/Polkadot; [working with Zcash foundation](https://www.parity.io/parity-teams-up-with-zcash-foundation-for-parity-zcash-client/)

* using Umbral Proxy Re-Encryption for message serialization

* Building Layer 2 Scaling Solutions on Parity; increasing the number of roles via the establishment of relayer nodes (creating a network of relayers could enable Griffith-style universal logins to fix UI problems involved with gas)

> Light clients will be the backbone of decentralized applications in the near term and this is very good news for a user-friendly decentralized ecosystem.

* Researching the incentivization of running full nodes; make light clients perform micro-payments for each request made to full nodes (micropayments can be made cheap by utilizing state channels).

* Research on allowing light clients to sync quickly while avoiding the current tradeoffs. One idea is to allow full nodes to provide zero-knowledge proofs (e.g. zk-STARKs) of the latest known header. The light client could then verify this and sync with the top of the chain without prior knowledge of the blockchain state.

* better bounty program for auditing critical code; sun is the best disinfectant