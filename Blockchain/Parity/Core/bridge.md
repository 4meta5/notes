# Bridge

* Bridges work by having a light client on both sides of the bridge -- the light client interprets whatever state transitions have happened and introduces them as extrinsics (~transactions) into the other side of the bridge. This lets the other side interpret what is going on.
* Security guarantees are analagous to the security guarantees of the light client. The biggest problem with bridges would arise if you were bridging a chain with absolute finality to a chain with less of a finality guarantee...so then you use probabilistic finality as a tacit guarantees 

> [github](https://github.com/paritytech/parity-bridge)

parity-bridge is currently an ERC20 token contract on one ethereum-based blockchain that is backed by ether on another ethereum-based blockchain.

eventually parity-bridge will be able to pass arbitrary messages between two ethereum-based blockchains. in the future you'll be able to build the current ether-ERC20 bridge and any other cross-chain application on top of the message passing bridge.

currently users can convert ether on one chain into the same amount of ERC20 tokens on the other and back. the bridge securely relays these conversions.

the bridge can mitigate scaling issues: by deploying a proof-of-authority network and bridging it to the Ethereum Foundation network ('mainnet') users can convert their mainnet ether into ERC20 tokens on the PoA chain and there transfer them with much lower transaction fees, faster block times and unaffected by mainnet congestion.