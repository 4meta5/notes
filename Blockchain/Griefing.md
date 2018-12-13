# Griefing Factor Analysis

In [The Triangle of Harm](https://vitalik.ca/general/2017/07/16/triangle_of_harm.html), Vitalik reduces the relevant actors for blockchain security to the minority, the majority, and the protocol (ie users). With these actors in mind, Vitalik identifies four categories of attack:
1. Minority attacking the protocol, i.e. [finney attack](https://bitcoin.stackexchange.com/questions/4942/what-is-a-finney-attack)
2. Minority attacking the majority, i.e. [feather forking](https://bitcointalk.org/index.php?topic=312668.0)
3. Majority attacking the protocol, i.e. 51% attacks
4. Majority attacking the minority, i.e. censorship attack

> For all four categories of attack, we want to put an upper bound on the ratio between the amount of harm suffered by the victims of the attack and the cost to the attacker (which provides the intuition behind *griefing factor analysis*)

The **griefing factor** of a strategy is essentially the amount of money lost by the victims divided by the amount of money lost by the attackers. The griefing factor of a protocol is the highest griefing factor that it allows.

> whenever a speaker/listener dichotomy exists (*data availability problem*), the griefing factor cannot be globally bounded by any value below 1

Proof of work consensus punishes dissent and, by construction, does not allow us to bound the griefing factor for **3** or **4**. Conversely, proof of stake punishes equivocation; by requiring validators to post collateral, we can utilize slashing to *penalize both sides* during specified faults (and use this to bound the griefing factor in a near-optimal way).

## Extortion Griefing Bounds

* [A Note on Limits on Incentive Compatibility and Griefing Factors](https://vitalik.ca/files/extortion_griefing_bounds.pdf)

## Level K Attack
> take notes on this recent attack

### References
* [A Note on Limits on Incentive Compatibility and Griefing Factors](https://vitalik.ca/files/extortion_griefing_bounds.pdf)
* [The Triangle of Harm](https://vitalik.ca/general/2017/07/16/triangle_of_harm.html)