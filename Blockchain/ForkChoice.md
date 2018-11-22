# Fork Choice Rules

GHOST (Greedy Heaviest Observed Subtree):
* A block must specify its parents and its number of Uncles
* An Uncle included in a block must be a direct child of the new block and less than seven blocks below it in terms of height
* It cannot be the direct ancestor of the block being formed.
* An Uncle must have a valid block header.
* An Uncle must be different from all other Uncles in previous blocks and the block being formed.
* For every Uncle included in the block the miner gets a additional 3.125% and the miner of the Uncle receives 93.75% of a standard block reward.

LMD GHOST fork choice: Let M be the set of most recent messages from all validators. Set HEAD = genesis. Loop: set HEAD to the child C of HEAD such that the largest subset of M votes for C or a descendent of C, repeat until HEAD has no choldren. Return HEAD.

### References

* [Secure High-Rate Transaction Processing in Bitcoin](https://eprint.iacr.org/2013/881.pdf)
* [PoS fork choice rule desiderata](https://ethresear.ch/t/pos-fork-choice-rule-desiderata/2636)