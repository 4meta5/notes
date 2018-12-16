# Information Theory

* [Erasure Encoding Notes](./ErasureEncoding.md)
* [Error Correction Notes](./ErrorCorrection.md)

## Intro to Information Theory

There are two basic problems central to Information Theory.

**Noiseless coding problem**: an encoding scheme for messages which minimizes the expected length of an encoded message and guarantees the ability to unambiguously decode a message.

> motivation: communication is expensive e.g. asynchronous communication

**Noisy coding problem**: an encoding scheme that guarantees with high probability the ability to decode any sent message

> motivation: communication is inexpensive, but error prone

Interestingly, both of these problems were posed and solved in the same [paper](https://en.wikipedia.org/wiki/A_Mathematical_Theory_of_Communication) by Claude Shannon in 1948 (he opens by introducing the concept of entropy with markov chains to model the english language).

*Entropy* refers to the "information content" of a message (<=> the expected "compressibility" of a single bit under the best encoding). 

> Think of the domain of the distribution as being compressed, not the specific string itself (Kolmogorov complexity is the term for the compressibility of a specific string)

More formally, suppose `D` is a distribution on a finite set `X`, and we'll use `D(x)` to denote the probability of drawing `x` from `D`. The **entropy** of `D`, denoted `H(D)` is defined as 

```
H(D) = \sum_{x \in X} D(x) log(\frac{1}{D(x)})
```

Therefore, a high entropy measures how *incompressible* something is, while a low entropy provides lots of compressibility. Shannon's *Noiseless Coding Theorem* shows that that the entropy of the distribution is the only thing that matters.

**Noiseless Coding Theorem**: For every finite set `X` and distribution `D` over `X`, there are encoding and decoding functions `Enc: X -> {0, 1}^{*}, Dec: {0, 1}^{*} -> X` such that
1. The encoding/decoding actually works, i.e. `Dec(Enc(x)) = x` for all `x`
2. The expected length of an encoded message is between `H(D)` and `H(D) + 1`.

Most importantly, *no encoding scheme can do better*! [Proof](https://en.wikipedia.org/wiki/Shannon%27s_source_coding_theorem)

The interpretation of the **Noisy Coding Problem** (which has not been completely solved/optimized yet, and is an active area of research in coding theory) is that you want to be able to recover from white noise errors introduced during transmission. The concept is called *error correction*.

> We want to recover from error with probability asymptotically close to 1, where the probability is over the errors

**Noisy Coding Theorem**: For any constant noise rate `p < 1/2`, there is an encoding scheme,

```
Enc: {0, 1}^{k} -> {0, 1}^{ck}
Dec: {0, 1}^{ck} -> {0, 1}^{k}
```

such that if `x` is the message sent by Alice and `y` is the message received by Bob (i.e. `Enc(x)` with random noise), then `Pr[Dec(y) = x] -> 1` is a function of `n = ck`.

Moreover, if we denote `H(p)` the entropy of the distribution of an error on a single bit, then choosing any `c > \frac{1}{1 - H(p)}` guarantees the existence of such an encoding scheme, and no scheme exists for any smaller `c`.

The proof of this theorem is probabilistic. Specifically, Shannon proved such an encoding scheme exists by picking `Enc` to be a random function `!`. Then `Dec(y)` finds `Enc(x)` and `y` is minimized. The difference in the number of bits is called the *Hamming distance*.

### References
* [MIT: Algorithmic Introduction to Coding Theory](http://people.csail.mit.edu/madhu/FT01/)
* [Markov Chains](https://en.wikipedia.org/wiki/Markov_chain#Information_and_computer_science)

> Markov chains are used throughout information processing. Claude Shannon's famous 1948 paper A Mathematical Theory of Communication, which in a single step created the field of information theory, opens by introducing the concept of entropy through Markov modeling of the English language. Such idealized models can capture many of the statistical regularities of systems. Even without describing the full structure of the system perfectly, such signal models can make possible very effective data compression through entropy encoding techniques such as arithmetic coding. They also allow effective state estimation and pattern recognition. Markov chains also play an important role in reinforcement learning.

**Jeremy Kun's Blog**
* [A Proofless Introduction to Information Theory](https://jeremykun.com/2015/02/16/a-proofless-introduction-to-information-theory/)
* [Hamming's Code](https://jeremykun.com/2015/03/02/hammings-code/) 
* [The Codes of Solomon, Reed, and Muller](https://jeremykun.com/2015/03/23/the-codes-of-solomon-reed-and-muller/)
* [The Welch-Berlekamp Algorithm for Correcting Errors in Data](https://jeremykun.com/2015/09/07/welch-berlekamp/)