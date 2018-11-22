# Ethereum 2.0 Notes

Ethereum 2.0 incorporates the Casper consensus mechanism and Sharding. 

## The Beacon Chain

The beacon chain stores and manages the set of active proof-of-stake validators. This is the central PoS chain that serves as the base of the sharding system. 

**How to become a validator:**
* make a fixed-size one-way ETH deposit to a registration contract on the Ethereum 1.0 PoW chain (currently set at 32 ETH)
* if the registration transaction receipts are processed by the beacon chain, the validator is inducted on the beacon chain

The beacon chain serves to strengthen the security of the network by storing attestations of transactions from validators to the data stored on sharded blocks. **Attestations** attest to a shard block as well as the corresponding beacon chain block. 

If a shard block achieves a sufficient number of attestations, it creates a **crosslink**, thereby confirming the given shard segment up to that shard block into the beacon chain. Likewise, crosslinks serve as infrastructure for asynchronous cross-chard communication. Indeed, crosslinks are the main means by which the beacon chain "learns about" the updated state of shard chains.

**Specific Terminology**<br>
* A **slot** is a period of ```SLOT_DURATION``` seconds, during which oen proposer has the ability to create a beacon chain block while attesters maintain the ability to make attestations.
* A **cycle** is a span of slots when all validators have exactly one chance to make an attestation
* The withdrawal period specifies the number of slots between a validator exit and when the validator balance is withdrawable

The beacon chain's main responsibilities are:
1. store and maintain the set of active, queued and exited validators
2. process crosslinks
3. process its own block-by-block consensus (and the finality gadget)

For a block on the beacon chain to be processed by a node, four conditions must be met:
1. The parent pointed to by the ```ancestor_hashes[0]``` has already been processed and accepted
2. An attestation from the *proposer* of the block is included along with the block in the network message object
3. The PoW chain block pointed to by the ```pow_chain_reference``` has already been processed and accepted
4. The node's local clock time is greater than or equal to the minimum timestamp as computed by the ```GENESIS_TIME + block.slot * SLOT_DURATION```

> (4) implies that each node must have a clock that is roughly synchronized with the other nodes

> question: can an incorrectly built client manipulate (4)?

## Fork Choice Rule

The beacon cahin uses the Casper FFG fork choice rule of "favor the chain containing the highest-slot-number justified block". To choose between chains that are all descended from the same justified block, the chain relies on "immediate message driven GHOST" (IMD GHOST) to choose the head of the chain.

## State Transition Function

The state transition is made up of two parts:
1. The per-block processsing (which occurs every block) and affects the ```ActiveState``` only
2. The crystallized state recalculation, which happens only if ```block.slot >= last_state_recalculation_slot + CYCLE_LENGTH```, and affects the ```CrystallizedState``` and ```ActiveState```.

> the crystallized state recalculation generally focuses on changes to the validator set (including adjusting balances as well as adding/removing validators). In addition, it entails processing crosslinks and managing block justification/finalization

> The per-block processing generally focuses on verifying aggregation signatures and saving temporary records relating to the in-block activity in the ```ActiveState```.

### Transitioning from Proof of Work

A registration contract is added to the PoW chain to deposit ETH ([more details](https://github.com/ethereum/eth2.0-specs/blob/master/specs/beacon-chain.md#pow-chain-registration-contract)).

