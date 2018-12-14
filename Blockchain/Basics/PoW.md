# Proof of Work Consensus

At a high-level, these are the basic steps of Nakamoto Proof of Work consensus:
1. Transactions are broadcast to the decentralized network
2. Nodes in the network group the transactions into blocks.
3. The nodes compete with each other to decide who gets to verify this block of transactions.
    * the competition consists of hashing the block of transaction data along with some random number so that the output string fulfills some requirement; more specifically, it needs to have a certain number of leading zeros
    * The hashed output is called the **proof-of-work** and has the characteristics of being difficult to compute and impossible to fake.
4. The proof-of-work is published to the network along with the block of transactions and nonce so that anyone can check that it is indeed a valid proof-of-work
5. The validated block is added to the blockchain, which is a chain of the entire history of validated transactions.
6. Nodes are rewarded for eac h block that they find with an amount of newly created Bitcoin and/or fees, as incentive for validating transactions and securing the network.
7. The competition begins again for validating the next block of transactions. 