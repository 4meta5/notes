# [Insert Clever Governance Title Here]

We're going to use this podcast to discuss governance. 

> [Vlad and Gav go head-to-head on blockchain governance](https://www.zeroknowledge.fm/52)

SCOPE: focus on this podcast; don't make it too broad...point people to Steve D. McKie's article for a more extensive analysis on the spectrum of governance models!

## In defense of Vlad Zamfir

> rough consensus; keep things informal

— when I initially listened to vlad vs gav, I thought gav destroyed vlad. It seemed to me at the time that Vlad was conservative and Gav was progressive.

— recently reading Cyperpunks, and on page 106, I came across something that made me reevaluate my previous interpretation of the conversation. When we formalize governance prematurely, we disenfranchise future stakeholders and take away potentially more efficient, informal governance mechanisms that may arise organically from stakeholder interaction with the system (blockchain in this case).

— there’s a balance here that needs to be considered, and this balance is important. Goldilocks principle...in my opinion, the best approach will be a diversity of networks with different shades of formalized governance. See McKie and Pruitt recent blog post.

— consider the limitations of coinholder voting for blockchain governance decisions.
——Parity Light Protocol is designed to incorporate light clients and may entail incentivizing full nodes via micropayment models. Parity’s substrate protocol also utilizes Collators in addition to Validators; these collators are also tasked as fishermen. In the future, we don’t know what kinds of stakeholders may surface, and, if we restrict voice in the context of governance to coinholders, then we preemptively disenfranchise these future stakeholder groups, thereby limiting what we can achieve in the future. The best way forward is to try and expand our options — this is also the best strategy in chess.

**Advantages**
* prevents preemptively formalizing governance <=> disenfranchising future stakeholders

**Disadvantages**
* gridlock
* centralization in power; Hudson Jameson LOL

## Polkadot's Approach

> Can we make governance dynamic such that we formalize governance, while still enabling the changes necessary to progress and update in a smooth way? 

> "any governance that depends on high turnout is doomed to fail" - Fredrik

Discussion of adaptive quorum biasing -- pros and cons?

One counterargument is that current stakeholders rarely have the incentive to cede power or change the existing power structure.

### References
* [Blockchain Communities and Their Emergent Governance](https://medium.com/amentum/blockchain-communities-and-their-emergent-governance-cfe5627dcf52) by Matthew Prewitt and Steven McKie

**Vlad Zamfir**
* [Blockchain Governance 101](https://blog.goodaudience.com/blockchain-governance-101-eea5201d7992)
* [My Intentions for Blockchain Governance](https://medium.com/@Vlad_Zamfir/my-intentions-for-blockchain-governance-801d19d378e5)
* [Against On-Chain Governance](https://medium.com/@Vlad_Zamfir/against-on-chain-governance-a4ceacd040ca)

**Gavin Wood**
* [Epicenter: Substrate, Polkadot and the Case for On-Chain Governance](https://www.youtube.com/watch?v=eP4mT19S_jg)

*automated upgrades are a sharp knife* -- [Epicenter Interview with Gavin](https://www.youtube.com/watch?v=eP4mT19S_jg)

> sharp knife is dangerous but a good chef prefers a sharp knife to a dull knife; we can do much more (use it to close the section)

> need to ensure that the upgrade fits within your chain definition and there are an appropriate number of safeguards to prevent them from being misused

on-chain governance: extend consensus technology to dictate not only what happens on the chain but what happens with the chain as well

Substrate is a meta-version of Ethereum; instead of having a smart contract that dictates a state transition function,we store a runtime on chain which dictates the state transition function...this model can be viewed as enabling greater *smart contract* upgradeability

> how do we upgrade our on-chain governance in this case? 

Caveats: we need to design correct fail-safes

on-chain governance is a strictly defined process; off-chain governance is too ambiguous and often leads to an abuse of power

"Bitcoin is controlled by Bitcoin Core and 7-8 miners. Ethereum is essentially a dictatorship -- if Vitalik wants to do something, then it is done."

Plutocracy/Oligarchy vs Dictatorship

> stakeholder voting by itself is an improvement

> design the coin distribution mechanism so that no oneparty controls more than 10% of the coins

adage/maxim: if >50% of coinholders votes to make a change, the change is made (not 51+% of the turnout, 51+% of the coinholders)

council: voted in through apprval voting; by deferring to a smaller set of people, they can bring about and propose sensible changes; still with a demographic safeguard

idea expressed...when you vote for a change, you lock your coins up for some period after the change is implemented...

idea expressed...incorporating forking into governance (where if a vote goes through, the minority's stake is slashed but they fork onto a new chain on which the majority's stake is slashed)...what does forking mean in the context of polkadot though?

*"It's very easy to fork"* -- I really think this is a misperception.