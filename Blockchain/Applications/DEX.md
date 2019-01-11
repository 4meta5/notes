# Decentralized Exchanges

* [inDEX Metalink](https://github.com/distribuyed/index/blob/master/README.md)
* [Messari Decentralized Exchange Resources](https://messari.io/resource/decentralized-exchanges)
* [Circle Exchange Decentralized Exchanges](https://www.circle.com/en/research/decentralized-exchanges)

Different types of DEXs:
* ​Off-chain order book​
    * 0x
        * Radar Relay
        * Paradex
* Peer-to-peer​
    * Bisq
* Pool-based​
    * Ether Delta
    * IDEX
    * Bancor
    * Kyber
    * Token Store
    * Airswap
* Side-chain order book​
    * OMG

## Attack: Front Running

*High-level Algorithm*
1. Gain access to market order information that will increase the price of X.
2. Buy X before the guaranteed price increase.
3. Sell X after the guaranteed price increase.
4. Profit.

* [Decentralized Exchanganges have a problem: front-running](https://blog.gnosis.pm/decentralized-exchanges-have-a-problem-7e6d81d91ba1)


Possible Solution: **Batch Auctions**<br>
The batch auction format eliminates the lag that front-runners need to profit, by settling all trades for a given auction period simultaneously. Critically, this solution does so without conceding full-stack decentralization.

The chief downside of the batch auction format is that exchange requests cannot be executed quickly. Moving in or out of a position on a centralized exchange takes seconds but with a batch auction format, it could take minutes to hours.
