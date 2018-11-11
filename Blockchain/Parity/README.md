# Parity
> Here is where I'll organize some of my ideas related to [Parity](https://parity.io) -- you can read more about what I like about Parity [here](./<3.md)

> Parity wiki

* [Miscellaneous](#Miscellaneous)
* [Frontend](#Frontend)
* [Substrate](#Substrate)
* [Polkadot](#Polkadot)

## Miscellaneous

* [Althea Mesh Bidirectional Payment Channels](https://github.com/althea-mesh/guac_rs)

* using Bellman repo for Zero Knowledge constructions on Substrate/Polkadot; [working with Zcash foundation](https://www.parity.io/parity-teams-up-with-zcash-foundation-for-parity-zcash-client/)

* using Umbral Proxy Re-Encryption for message serialization

* Building Layer 2 Scaling Solutions on Parity; increasing the number of roles via the establishment of relayer nodes (creating a network of relayers could enable Griffith-style universal logins to fix UI problems involved with gas)

## Frontend
> [More Info on Parity's Frontend Code](./frontend.md)

* smart contracts on light clients (seems like the way things will work)

## Substrate
> [More Info on Substrate](./substrate.md)

* Rhododendron module

* Shasper (Serenity) module --> optimize SSZ

* MimbleWimble consensus module
* Coda consensus module (scaling transaction throughput with zero knowledge proofs -- see Bellman in Misc)

## Polkadot
> [More Info on Polkadot](./polkadot.md)

"Blockchain designed for scalability and interoperability. We have a root relay chain which manages the advancement and security of various parachains, which have their own state machines and can communicate amongst eachother"