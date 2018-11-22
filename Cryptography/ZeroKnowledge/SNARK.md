# SNARKs
> my personal notes from Zcash zk-SNARKs [explainer series](https://z.cash/technology/zksnarks/)

> need to make this explanation better!

* [Homomorphism](#HH)
* [Blind Evaluation of Polynomials](#Blind)
* [Knowledge of Coefficient Test](#KC)
* [Verifiable Blind Evaluation](#Verify)
* [Computations to Polynomials](#QAP)

## Homomorphism <a name = "HH" ></a>

`E(x)` is a homomorphic function if
1. given `E(x)`, we cannot find `x`
2. `x != y` => `E(x) != E(y)`
3. `E(x) + E(y) = E(x + y)`

Consider the group `Z_{p}` such that `p` is a prime (we keep the group operation undefined for now). By definition, this group is cyclic => there is some element `g` in `Z_{p}` referred to as the generator which can represent all elements in the group (if raised to a certain power).

The discrete logarithm problem is hard in `Z_{p}` <=> when `p` is large, given an element `h` in `Z_{p}`, it is difficult to find an integer `a` in `{0, ..., p - 2}` s.t. `g^{a} = h (mod p)`.

Now, we can define `E(x) = g^{x}`; `E(a + b) = g^{a} * g^{b} = g^{a + b} = E(a) + E(b)`. By the discrete logarithm problem, our group operation (exponentiation) fulfills (1). Moreover, the properties of this homomorphism ensure (2) and, as we have already shown, (3).

## Blind Evaluation of Polynomials <a name = "Blind" ></a>
We define a polynomial `P` of degree `d` over `F_{p}` as a linear combination of elements in `F_{p}` of the form: `P(X) = a_{0} + a_{1} * X + a_{2} * X + a_{2} * X^{2} + ... + a_{d} * X^{d}` such that the coefficients are in the given field.

If we use `E(x) = g^{x}`, we can easily see that `E(x)` supports linear combinations: `E(x)^{a} + E(y)^{b} = (g^{x})^{a} * (g^{y})^{b} = g^{ax} * g^{by} = g^{ax + by} = E(ax + by)`.

Let's suppose Alice has a polynomial `P` of degree `d`, and Bob has a point `s` in `F_{p}` that he chose randomly. Bob wishes to learn `E(P(s))`. We want Bob to learn `E(P(s))` without learning `P` and similarly we do not want Alice to learn `s`. What is the solution?
1. Bob sends Alice `E(1), E(s), ..., E(s^{d})`
2. Alice computes `E(P(s))` from the elements sent by Bob and sends `E(P(s))` to Bob (Alice can do this since `E` allows linear combinations and `P(s)` is a linear combination of `1, s, ..., s^{d}`)

The intuition behind SNARKs is that the verifier has a "correct" polynomial in mind, and wishes to check the prover knows it. Making the prover blindly evaluate their polynomial at a random point not known to them ensures the prover will give the wrong answer with high probability if the polynomial is not the correct one. 

> Schwartz-Zippel Lemma: *different polynomials are different at most points*

## Knowledge of Coefficient Test <a name = "KC"></a>
The Knowledge of Coefficient Test serves to ensure Alice follows the protocol described above correctly.

Define a group `G` of order `p` in which the discrete log is hard. For the purpose of this exercise, let's say the group operation is addition rather than multiplication (so for `\alpha` in `F_{p}`, `\alpha * g` denotes summing `\alpha` copies of `g`).

Let `\alpha` in `F_{p}`. We call a pair of elements `(a, b)` in `G` an `\alpha`-pair if `a, b !+ 0` and `b = \alpha + a`.

The KC test proceeds as follows:
1. Bob chooses random `\alpha` in `F_{p}` and `a` in `G`. He computes `b = \alpha * a`
2. He sends to Alice the *challenge* pair `(a, b)`. Notice that `(a, b)` is an `\alpha`-pair
3. Alice must now respond with a different pair `(a', b')` that is also an `\alpha`-pair
4. Bob accepts Alice's response only if `(a', b')` is indeed an `\alpha`-pair (he can easily check if `b' = \alpha * a'`)

Because Alice only knows `\alpha * a` in `G`, she cannot compute `\alpha` because `G` is a group in which the discrete log is hard.

**Knowledge of Coefficient Assumption (KCA)**: If Alice returns a valid response `(a', b')` to Bob's challenge `(a, b)` with non-negligible probability over Bob's choices of `a`, `\alpha`, then she knows `\gamma` such that `a' = \gamma * a`.

How do we formalize that "Alice knows `\gamma`"?

We create another party called *Alice's Extractor*, which has access to Alice's inner state. We can then reformulate the KCA: whenever Alice successfully responds with an `\alpha`-pair `(a', b')`, Alice's Extractor outputs `\gamma` such that `a' = \gamma * a`.



## Verifiable Blind Evaluation <a name = "Verify"></a>
We need to use *KCA* to achieve the *verifiable* blind evaluation of a polynomial.

So we need to restate the problem with our new desired properties. Alice has a polynomial `P` of degree `d` and Bob has a point `s` in `F_{p}` that he chose randomly. We want to construct a protocol to enable Bob to learn `E(P(s))` with the following desirable properties:
1. **Blindness**: Alice will not learn `s` and Bob will not learn `P`
2. **Verifiability**: The probability that Alice sends a value not of the form `E(P(s))` for `P` of degree `d` that is known to her, but Bob still accepts is negligible

The intuition is that Alice commits to an *answer polynomial* without seeing the *challenge point* `s`.

We extend the KCA => **d-power Knowledge of Coefficient Assumption (d-KCA)**: Suppose Bob chooses random `\alpha` in `F_{p}` and `s` in `F_{p}`, and sends Alice the `\alpha`-pairs `(g, \alpha * g), (s * g, \alpha * s * g), ..., (s^{d} * g, \alpha * s^{d} * g)`. Suppose that Alice then outputs another `\alpha`-pair `(a', b')`. Then, except with negligible probability, Alice knows `c_{0}, ..., c_{d}` in `F_{p}` such that `\sum_{i = 0}{d} c_{i}s^{i} * g = a'`.

> Note that in d-KCA Bob doesn't send an arbitrary set of `\alpha`-pairs but one with a certain polynomial structure.

Define `E(x) = x * g` for a generator `g` of `G`.

The protocol for this particular `E`:
1. Bob chooses a random `\alpha` in `F_{p}` and sends to Alice the hidings `g`, `s * g`, ..., `s^{d} * g` and also the hidings `\alpha * g, \alpha * s * g, ..., \alpha * s^{d} * g`
2. Alice computes `a = P(s) * g` and `b = \alpha * P(s) * g` and sends the result to Bob
3. Bob verifies that `b = \alpha * a` and accepts iff this equality holds

This shows how we can use d-KCA for verifiable blind evaluation of polynomials.

## Computations to Polynomials <a name = "QAP"></a>
> [Oh QAP, finish these notes later](https://z.cash/blog/snark-explain5)
Suppose Alice wants to prove to Bob she knows `c_{1}, c_{2}, c_{3}` in `F_{p}` such that `(c_{1} * c_{2}) * (c_{1} + c_{3}) = 7`. The first step is to present the expression computed from `c_{1}, c_{2}, c_{3}` as an *arithmetic circuit*.

