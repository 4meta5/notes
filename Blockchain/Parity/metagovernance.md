# Metagovernance
> There are number of claims made here that are not 100% true. For example, we can still execute hard forks for binding on-chain governance protocols, but I ignore the nuances of such situations for the purpose of clearly communicating the main points...[Sue me](https://www.youtube.com/watch?v=lppTJFYigoU)

Blockchain protocols leverage a combination of cryptography, game theory, and network security to reach consensus on the ordering of transactions in a distributed setting. The value proposition of blockchain technology lies in its ability to incentivize cooperation among a network's stakeholders, even if the set of stakeholders isn't well-defined. By implementing a *permissionless* miner/validator set, blockchains can achieve *decentralization*, a necessary prerequisite for adequately distributing trust to claim *trustlessness*.

**Admittedly, many of these properties are hard to define and even more difficult to measure**. Because of the ambiguity of '*permissionless*', '*decentralized*', and '*trustless*', it is tempting to denounce these features as unrealistic, succumb to the pressure of scaling transaction throughput, and forego the initial vision. But similar to how no one entity controls the consensus mechanism, no one person should be able to determine which tradeoffs we make when updating blockchain protocols. With this in mind, the focus naturally shifts to reassessing how we govern blockchains themselves, an interesting variant of *[metagovernance](https://en.wiktionary.org/wiki/metagovernance)*.

In a [recent ZeroKnowledgeFM podcast](https://www.zeroknowledge.fm/52), Vlad Zamfir and Gavin Wood debate the merits of on-chain vs. off-chain governance. Gavin argues in favor of *on-chain governance*, which he defines as extending blockchain consensus technology to dictate not only what happens on the chain but also what happens with the chain. Vlad argues in opposition of formalized metagovernance and in support of *off-chain governance*. Under this model, informal discussions enable the community to signal preferences on software updates and come to *rough consensus* before making a decision.

At its core, we are asking the following question: *should we formalize blockchain governance with the same mechanisms we use to reach consensus on transaction data or should we keep blockchain governance informal?* In this post, we will discuss the metagovernance debate in the hopes of clearly communicating each side; in the end, things are much more nuanced than a binary determination of right or wrong.

## Off-Chain Governance via Rough Consensus
> *Treat your articulation of governance problems and proposals as a loaded weapon and don't shoot in the dark.* ~ Vlad Zamfir, [Against on-chain governance](https://medium.com/@Vlad_Zamfir/against-on-chain-governance-a4ceacd040ca)

Before we jump into the metagovernance debate, we should consider the stakeholders involved in maintaining public blockchains. 

At the consensus level, **miners/validators** participate in an algorithm to organize transactions into blocks and commit blocks to the blockchain. Consensus algorithms implement an [anti-sybil](https://bitcoin.stackexchange.com/questions/50922/whats-a-sybil-attack) mechanism in order to attach a cost to the establishment of identity (corresponding to computational power in Proof of Work and coins staked in Proof of Stake). Indeed, the anti-sybil mechanism is a necessary part of any on-chain governance protocol because, without it, malicious actors can create an arbitrary number of identities to bias the outcome of a vote. 

In addition to miners/validators, there are number of other significant actors that support critical blockchain infrastucture. Some organizations and individuals run **full nodes** to enable secure user interaction with the blockchain. Using these full nodes as intermediaries, users may run **light clients**. Although light clients don't directly help the network, they provide a smooth end-user experience without incurring the overhead required to maintain a full node. Other blockchain stakeholders include DApp **developers**, DApp **users**, and even **coin hodlers**. For each of these stakeholder groups (with the exception of coin hodlers), there is no known anti-sybil mechanism to enable participation in on-chain governance protocols.

In the *rough consensus* model of off-chain governance, the community hosts regular developer, client, and research-specific video calls to gauge the sentiment of key stakeholders. Moderators such as Hudson Jameson are tasked with interpreting the lack of objection among participants to justify decisions made on behalf of the greater community. Although this style of rough consensus is far from optimal, it is arguably better than an on-chain protocol in which stakeholders are entirely disenfranchised due to the lack of a viable anti-sybil mechanism. 

In the on-chain governance paradigm, full nodes automatically update when an on-chain governance process decides on an upgrade, regardless of whether their preferences were ultimately taken into account. As previously mentioned, full node operators are necessary supporters of blockchain infrastructure that deserve a voice in the metagovernance process. In the off-chain governance model, node operators that dissent with a protocol decision can avail themselves of the increased optionality provided by *[hard forks](https://medium.com/@Vlad_Zamfir/dear-ethereum-community-acfa99a037c4)*. Binding on-chain governance robs node operators of this choice by forcing them to follow on-chain processes in which their interests are not represented. Indeed, "defaults are incredibly powerful" [1](https://medium.com/@Vlad_Zamfir/against-on-chain-governance-a4ceacd040ca).

Looking forward, it is nearly impossible to predict the next major innovation. In the blockchain ecosystem, the pace of development moves considerably faster than any established policy-making process. Likewise, when we formalize metagovernance based on how the blockchain works today, we tacitly strengthen and weaken certain power dynamics between existing stakeholders, thereby limiting the future participation of new stakeholders which may make the entire process more efficient.

To conclude, just because we're making collective decisions does not necessarily mean we agree on the criteria for making those decisions. *Rough consensus* is predicated on **thoughtful disagreement**; even in the midst of controversy, we can create open forums to communicate our thoughts and signal our preferences. In the end, stakeholders are not affected by governance decisions in proportion to their coin holdings. With this in mind, we need to maintain informal off-chain governance processes to represent as many people as possible, while still keeping everything open and *permissionless*.

In summary, the main arguments for off-chain governance via *rough consensus* include:
1. Off-chain governance incorporates the preferences of stakeholders (ie node operators) for which there does not exist a known anti-sybil mechanism.
2. Binding on-chain governance is coercive; it forces full nodes to follow the outcome even if their preferences are not take into account.
3. When we formalize governance prematurely, we disenfranchise future stakeholders and take away potentially more efficient, informal governance mechanisms that may arise organically from stakeholder interaction with the system.
> Although the lack of an anti-sybil mechanism prevents the formalization of key stakeholder participation in governance, *rough consensus* through community discussions can offer an informal way for blockchain clients (and other important stakeholders) to voice their preferences.

## Binding On-Chain Governance
> *Automatic upgrades are a sharp knife* ~ Gavin Wood, [Epicenter #259](https://www.youtube.com/watch?v=eP4mT19S_jg)

1. informal governance is a red herring (look at phone for more details)...human beings are by definition biased; we need to stop creating centralized points of trust based on some abstract notion of *legitimacy*; this model is very susceptible to abuse and is not dissimilar to the existing models for transacting and managing user interactions which blockchain servces to circumvent at the consensus layer

- If there is a power law distribution and only a few forks are ever successful, then do we really have a choice to fork blockchains? We treat this as if it is a bona fide mechanism for voicing dissent when, in reality, it represents a false choice. If I disagree with a protocol upgrade and am in the minority, it is probably not worth dissenting if it means sacrificing the network effects enjoyed by the existing community. Sure, I can stand by my principles, but if that means losing capital as well as future support from core developers, it seems obvious that the choice never really existed. Instead of pretending that hard forks offer users a legitimate voice, why can't we accept that informal governance is often just as coercive as mandatory software upgrades?

- I guess the legitimate question would be whether the hard fork is opt in or opt out. If it is opt in, then it seems they have a choice in which the default is to continue on their path and the opt in decision allows them to voice their support of the hard fork; but the DAO hard fork was opt out for most clients and, indeed, the organizations behind the main clients revealed that they would not continue updating the software for the other fork. When your decision is made for you, do you really have a decision?

- fear mongering on tap (Vlad); a lot of this sounds a lot like what people said before blockchains

- Hudson Jameson: some voices are louder than others; he is tasked as the interpreter

2. As we formalize the governance of these systems to eliminate the potential for abuse, it makes sense to vest power in the stakeholders who have skin in the game. This is the coinholders. They are held accountable by their investment in the protocol itself and, if they make coercive upgrades that are disadvantageous to other stakeholders, the price should go down and the market should punish them. Instead of relying on some undefined, vague notion of legitimate off-chain governance based on interpretation of collective opinions, we can use the market to hold coinholders accountable in their role as metagovernance agents.

3. We need to experiment with more live metaprotocols that can make progress. We can have varying shades of conservativism among the blockchain systems; let the market decide who is right...

> "any governance that depends on high turnout is doomed to fail" - Fredrik

Discussion of adaptive quorum biasing -- pros and cons?

4. We can mitigate the seemingly coercive nature of binding on-chain governance with a few additional protocols: (1) council of experts (2) ...other stuff below

In *Cypherpunks*, Jeremie Zimmerman argues that "policy has to adapt to society, and not the other way around". He expands on his point, claiming that "good policy looks at the world and adapts to it in order to correct what is wrong and enable what is good. I'm convinced that when you enable the most powerful industrial actors to decide what policy should be, you don't go that way".

In summary, the main arguments for *on-chain governance* include:
1. In practice, off-chain governance creates centralized points of trust that are susceptible to abuse.
2. It makes sense to vest responsibility for metagovernance decisions in the stakeholders that have *skin in the game* i.e. coin hodlers.
3. Experimentation is objectively good. We can experiment with varying shades of conservatism among metagovernance protocols to decide on the right balance.
4. We can mitigate the seemingly coercive nature of binding on-chain governance with additional protocols.
> Some dope end sentence here

## WHO IS RIGHT?
> discussion of clash!

The premise is incorrect here!

— there’s a balance here that needs to be considered, and this balance is important. Goldilocks principle...in my opinion, the best approach will be a diversity of networks with **different shades of formalized governance**. See McKie and Pruitt recent blog post.

> what if we did develop a protocol for enabling sybil resistant measurement of full node operators based on the number of light clients that they serve? There is a measureable cost here and we can gauge voting power accordingly? A big research question is developing an anti-sybil mechanism for full node operators; I personally think that this could be managed via some proof of service to light clients. If we create a protocol for micropayments to incentivize full node operation (via payments from light clients to the full nodes), then we can use the proof of service as an anti-sybil mechanism to enable full node participation in on-chain governance. 

# References
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

> I have a theory that Vlad is actually a scapegoat designed to push the community towards on-chain governance.

- *the price of freedom is constant capitulance* ~ Gavin Wood