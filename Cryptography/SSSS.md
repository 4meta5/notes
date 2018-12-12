# Shamir's Secret Sharing Scheme

We can split a secret into `n` parts -- called secret shares -- and distribute them to different places/people. Later, we only need to collect `k` (`k > 0` and `k <= n`) secret shares to recover the original secret (`k` is the threshold).

**Definition**: A secret-sharing scheme involves a dealer who has a secret, a set of `n` parties, and a collection `A` of subsets of parties called the *access structure*. A secret-sharing scheme for `A` is a method by which the dealer distributes shares to the parties such that: (1) any subset in `A` can reconstruct the secret from its shares, and (2) any subset not in `A` cannot reveal any partial information on the secret.

There are a few implementations:
* [Polynomial Interpolation](#poly)
    * [Linear Algebra Approach](#linearschemes)
* [Chinese Remainder Theorem](#crt) 
    * [Asmuth-Bloom Scheme](https://en.wikipedia.org/wiki/Secret_sharing_using_the_Chinese_remainder_theorem#Asmuth-Bloom.27s_threshold_secret_sharing_scheme)
* [Blakely's Scheme](https://en.wikipedia.org/wiki/Secret_sharing#Blakley.27s_scheme)

> Note: Any time you have an existence and uniqueness theorem in mathematics, and the objects you're working with are efficiently constructible, then you have the potential for a secret sharing protocol.

A major problem wuth secret-sharing schemes is that the shares' size in the best known secret-sharing schemes realizing general access structures is exponential in the number of parties in the access structure. Thus, the known constructions for general access structures are *impractical*.

**Open Problem: Prove/Disprove the Following Conjecture**: There exists an `e > 0` such that for every integer `n` there is an access structure with `n` parties, for which every secret-sharing scheme distributes shares of length exponential in the number of parties, that is, `2^{e*n}`.

## Polynomial Interpolation <a name = "poly" ></a>
To distribute a secret to `k` people so that `r` of them can reconstruct it:
1. Pick a random polynomial `p` of degree `r-1` so that the secret is `p(0)`
2. Distribute the points `(1, p(1)), (2, p(2)), ... (k, p(k))`

The reconstruction function consists of taking the points provided by at least `r` participants, using them to reconstruct `p`, and outputting `p(0)`.

> polynomials can be encoded as vectors (where the vectors consist of either the coefficients or corresponding degrees for each term of the polynomial)...this provides the intuition for linear schemes (discussed below)

### Linear Schemes <a name = "linearschemes"></a>
In a linear scheme, the secret is viewed as an element of a finite field, and the shares are obtained by applying a linear mapping to the secret and several independent random field elements. 

> Linear secret sharing schemes are used in secure MPC protocols

## Chinese Remainder Theorem Method <a name = "crt"></a>
> assume knowledge of the [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)

* [link](https://en.wikipedia.org/wiki/Secret_sharing_using_the_Chinese_remainder_theorem)

### References
* [Survey Paper](https://www.cs.bgu.ac.il/~beimel/Papers/Survey.pdf)
* [The Mathematics of Secret Sharing](https://jeremykun.com/2014/06/23/the-mathematics-of-secret-sharing/) by Jeremy Kun
* [Eric Rafaloff Blog Post](https://ericrafaloff.com/shamirs-secret-sharing-scheme/)
* [Secret sharing using the Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Secret_sharing_using_the_Chinese_remainder_theorem)