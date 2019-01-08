# Construction

Computation => Arithmetic Circuit => R1CS => QAP => zk-SNARK

The first step of turning the *transaction validity function* into a mathematical representation is to break down the logical steps into the smallest possible operation to create an **Arithmetic Circuit**. 

The second step is to build a **Rank 1 Constraint System (R1CS)** to check that the values are traveling correctly through the arithmetic circuit. In the R1CS representation, the verifier has to check many constraints -- one for almost every wire of the circuit (interestingly, we only have a constraint for wires coming out of multiplication gates).

In [this paper](https://eprint.iacr.org/2012/215.pdf), Gennaro, Gentry, Parno, and Raykova presented a way to bundle all of the constraints into one by using a representation of the circuit now referred to as a **Quadratic Arithmetic Program (QAP)**. The single constraint that needs to be checked is now between polynomials rather than between numbers. Although the polynomials can be quite large, this is not a problem becuase when an identity does not hold between polynomials, it will fail at most points. With this in mind, we only have to check that two polynomials match at one randomly chosen point to verify the proof with high probability.

It is obvious that, if the prover knows in advance which point the verifier may choose to check, they could craft polynomials that are invalid but satisfy the identity at that point. By utilizing **zk-SNARKs**, we can evaluate the polynomials "blindly" (ie without knowing which point is being evaluated); this prevents the prover or verifier from knowing which point will be checked.

## Creating a Shielded Transaction
Zcash leverages zk-SNARKs to prove that the conditions for a valid transaction have been satisfied without revealing crucial information about the addresses or the values involved. The sender of a shielded transaction constructs a proof to show that, with high probability:
* the input values sum to the output values for each shielded transfer
* the sender proves that they have the private spending keys of the input notes, giving them the authority to spend
* the private spending keys of the input notes are cryptographically linked to a signature over the whole transaction, in such a way that the transaction cannot be modified by a party who did not know these private keys

For Zcash, the shielded equivalent of a UTXO is a *commitment*, and spending a commitment involves revealing a *nullifier*. Zcash nodes keep lists of all commitments that have been created, and all the nullifiers that have been revealed. Commitments and nullifiers are stored as hashes.

`Commitment = HASH(recipient address, amount, rho, r)`

When a shielded transaction is spent, the sender uses their spending key to publish a nullifier which is the hash of the secret number `rho` from an existing commitment that has not been spent, and provides a zero knowledge proof to demonstrate they are authorized to spend it. The requirement is that the hash is not already in the set of nullifiers tracking spent transactions kept by every node in the blockchain.

*Nullifier = HASH(spending key, rho)*

The zero knowledge proof verifies that
* for each input note, a revealed commitment exists
* the nullifiers and note commitments are computed correctly
* it is infeasible for the nullifier of an output note to collide with the nullifier of any other note