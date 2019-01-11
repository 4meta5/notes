# Uncle Mining Protocol

Uncle mining strategy consist of forcing you own blocks into uncles (blocks not in the best chain) in order to earn uncle rewards while preventing your blocks from contributing to the block difficulty adjustments. Uncle mining is a greedy strategy (or even it can be considered dishonest), as the greedy miners get monetary compensations while providing less of the expected service to the network: uncle mining does contribute to securing the network due to GHOST weighting, but does not contribute to increasing the network transaction processing power.

In GHOST, solved blocks that are not part of the best chain (stale blocks) can become uncles when they are referenced by blocks in the best chain. The references are stored in a new field in the each block header. When an uncle is referenced, the branch that referenced the uncle increases its weight by the same amount as the uncle was part of the best chain. The best chain is selected as the branch with higher weight.

In GHOST uncles have no other function and their transaction content is ignored. But Ethereum does more with uncles: uncles are paid a reward that is created from thin air specifically for that purpose. 

The mining strategy I propose is simply to mine uncles instead of blocks extending the best chain. To mine uncles only, a miner must make sure that the block he solves is not included in the best chain (the chain with higher weight). For example, the miner start mining a normal child but then wait until all peers have received a competing block (at the same height) before broadcasting the sibling mined. 

The theoretical results show that under certain optimal conditions uncle mining is profitable when the miner hashing power is over 12.5%.

* [Uncle Mining, an Ethereum Consensus Protocol Flaw](https://bitslog.wordpress.com/2016/04/28/uncle-mining-an-ethereum-consensus-protocol-flaw/)