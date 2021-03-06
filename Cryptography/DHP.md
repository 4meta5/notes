# Diffie-Hellman Problem (DHP)
> The motivation for this problem is that many security systems use one-way functions: mathematical functions that are fast to compute, but hard to reverse i.e. enable encrypting a message, but reversing the encryption (without the decryption key) is difficult. If solving DHP were easy, these systems would be broken. 

The **Diffie-Hellman Problem (DHP)** is stated informally as:<br>
Given an element `g` and the values of `g^{x}` amd `g^{y}`, what is the value of `g^{xy}`?<br>

## Computational Diffie-Hellman (CDH)
Consider a cyclic group `G` of order `q`. The CDH assumption states that, given `(g, g^{a}, g^{b})` for a randomly chosen generate `g` and random `a, b \in {0,...,q-1}` it is computationally intractable to compute the value `g^{ab}`.

CDH is computationally hard.

## Decisional Diffie-Hellman (DDH)
Consider a (multiplicative) cyclic group `G` of order `q`, and with generator `g`. The DDH assumption states that, given `g^{a}` and `g^{b}` for uniformly and independently chosen `a, b \in \mathbb{Z}_{q}`, the value `g^{ab}` looks like a random element in `G`.

This is formally stated by saying that the following two probability distributions are computationally indistinguishable:
* `(g^{a}, g^{b}, g^{ab})`, where `a` and `b` are randomly and independently chosen from `\mathbb{Z}_{q}`.
* `(g^{a}, g^{b}, g^{c})` where `a`, `b`, `c` are randomly and independently chosen from `\mathbb{Z}_{q}`.
Triplets of the first kind are often called *DDH triplet* or *DDH tuples*.

DDH is easy.

## External Diffie-Hellman (XDH)
The external Diffie-Hellman (XDH) assumption is a computational hardness assumption used in elliptic curve cryptography. the XDH assumption holds that there exists certain subgroups of elliptic curves which have useful properties for cryptography. Specifically, XDH implies the existence of two distinct groups `<G_{1}, G_{2}>` wwith the following properties:
1. The discrete logarithm problem (DLP) and the computational Diffie-Hellman problem (CDH) are instractable in `G_{1}` and `G_{2}`.
2. There exists an efficiently computable bilinear map pairing `e(_, _): G_{1} x G_{2} -> G_{T}`.
3. The decisional Diffie-Hellman problem (DDH) is intractable in `G_{1}`.

# Discrete Logarithm Problem (DLP)
**Discrete logarithms** are logarithms defined with regard to multiplicative cyclic groups. If `G` is a multiplicative cyclic group and `g` is a generator of `G`, then from the definition of cyclic groups, we know every element `h` in `G` can be written as `g^{x}` for some `x`. The discrete logarithm to the base `g` of `h` in the group `G` is defined to be `x`. For example, if the group is `\mathbb{Z}_{5}` and the generator is `2`, then the discrete logarithm of `1` is `4` because `2^{4} = 1 mod 5` (`=` should have three bars here, but we abuse notation). 

The **discrete logarithm problem (DLP)** states, given a group `G`, a generator `g` of the group and an element `h` of `G`, find the discrete logarithm to the base `g` of `h` in the group `G`.

DLP is not always hard (it depends on the groups).

### References
* [Short signatures from the Weil pairing](https://www.iacr.org/archive/asiacrypt2001/22480516.pdf) -- Boneh, Lynn, Shacham (AsiaCrypt 2001)
* [Short Group Signatures](http://crypto.stanford.edu/~dabo/papers/groupsigs.pdf) -- Boneh, Boyen, Shacham (AsiaCrypt 2004)
* Wikipedia