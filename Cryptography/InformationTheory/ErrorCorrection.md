# Error Correction
> how to detect and correct errors

A **code** `C` is just a subset of `{0, 1}^{n}` for some `n`. Elements of `C` are called *codewords*.

Let's say that we want to send messages of length `k` such that our messages are in `{0, 1}^{k}`. Then, we can consider a code `C` as the image of some encoding function `Enc: {0, 1}^{k} -> {0, 1}^{n}`. 

We ensure that `Enc` is an injective function such that no two messages get sent to the same codeword. Then, `|C| = 2^{k}` and we can call `k = log |C|` the *messages length* of `C` (even if we don't have an explicit encoding function).

The alphabet of the encoded messages is denoted by an arbitrary set `\sigma`. Therefore, a code `C` is a subset of tuples in `\sigma^{n}`, and we denote `q = |\sigma|`.

We denote the *minimum distance* of a code by `d`. This is defined to be the minimum **Hamming distance** between all distinct pairs of codewords, such that Hamming distance is the number of coordinates for which two tuples differ. 

We can formalize the above with notation.

**Definition**: A code `C` is called an `(n, k, d)_{q}`-code if
* `C \subset \sigma^{n}` for some alphabet `\sigma`
* `k = log |C|`
* `C` has minimum distance `d`
* the alphabet `\sigma` has size `q`

> The big questions in coding theory are (1) for which values of these four parameters do codes exist? (2) fixing any three parameters, how do we optimize the other one?

A code is called **linear** if it can be identified with a linear subspace of some finite-dimensional vector space. Moreover, it is linear if
* the zero vector is a codeword
* the sum of any two codewords is a codeword
* any scalar multiple of a codeword is a codeword

> The interesting thing about *linear codes* is that you can describe everything about a linear subspace by giving a basis for the space.

**Definition**: A *generator matrix* of a `(n, k, d)_{q}`-code `C` is a `k x n` matrix `G` whose rows form a basis for `C`.

The main benefit is that having a generator matrix allows one to encode messages `x \in {0, 1}^{k}` by left multiplication `xG`. Intuitively, we can consider the bits of `x` as describing the coefficients of the chosen linear combination of the rows of `G` (which uniquely describes an element of the subspace). 

**Definition**: Let `H^{T}` be a generator matrix for `C^{\perp}`. Then `H` is called a **parity check** matrix.

`H` is the basis for `C^{\perp}` as columns. This means it has dimensions `n x (n - k)`. Also, it has the property that `x \in C` <=> `xH = 0`. With this in mind, maintaining zero dot product with the columns of `H` validates membership in `C`.

Likewise, the benefit of maintaining a parity check matrix is efficient error detection: compute `yH` on the received message `y`, and if it is nonzero then there was an error. Even so, the parity check matrix only guarantees error detection if you have fewer errors than the minimum distance of the code.

The **Hamming weight** of a vector `x`, denoted `wt(x)`, is the number of nonzero entries in `x`.

**Proposition**: The minimum distance of a linear code `C` is the minimum Hamming weight over all nonzero vectors `x \in C`.

> there are more notes; these are taken almost directly from [Jeremy Kun's Blog](https://jeremykun.com/2015/03/02/hammings-code/)

### References
* [Hamming's Code](https://jeremykun.com/2015/03/02/hammings-code/) 
* [The Welch-Berlekamp Algorithm for Correcting Errors in Data](https://jeremykun.com/2015/09/07/welch-berlekamp/)