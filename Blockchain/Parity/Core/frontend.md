# Parity FrontEnd

* light.js
* fether

## Light.js

* [Parity/Light.js: high-level reactive JS library optimized for light clients](https://github.com/paritytech/js-libs/tree/master/packages/light.js)
    * One of the big value adds of the Substrate stack is that it provides first class light client compatibility from the beginning of development.
    * Moreover, conventional experience tells us that most users will be running light clients in lieu of computationally expensive full nodes. With this in mind, we can expect many applications to depend on interact with light clients, making this a relatively useful JS library within this ecosystem

## Fether
> a decentralizedm light client-based wallet

Parity Fether aims to be the lightest and simplest decentralized wallet. It supports Ether and ERC-20 tokens, and runs on top of Parity Ethereum light client. This allows smooth synchronization and interaction with the Ethereum blockchain, in a decentralized manner.

By default, Parity Fether alpha runs on the Kovan test network. You can receive free Kovan Ether by posting your address in the Kovan Faucet Gitter channel. Fether will download and launch Parity Ethereum node at startup if it's not found on the computer. You can also separately launch your Ethereum client, Fether will automatically connect to it.

Parity Fether connects to the light node using @parity/light.js, a Javascript library specifically crafted for wallets to connect with light clients.

> This wallet makes correct use of `light.js` (so it is useful in that regard).

* [recent blog post](https://www.parity.io/parity-fether-alpha-is-here-a-decentralised-light-client-based-wallet/)


### More

* polkadot.js
* 007-bonds
* Parity Light protocol to incentivize running light clients.