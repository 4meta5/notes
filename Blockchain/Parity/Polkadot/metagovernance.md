# Both Sides of the Metagovernance Debate

Blockchain protocols leverage a combination of cryptography, game theory, and network security to reach consensus on the ordering of transactions in a distributed setting. The value proposition of blockchain technology lies in its ability to incentivize cooperation among a network's stakeholders, even if the set of stakeholders isn't well-defined. By implementing a *permissionless* miner/validator set, blockchains can achieve *decentralization*, a necessary prerequisite for adequately distributing trust to claim *trustlessness*.

**Admittedly, many of these properties are hard to define and even more difficult to measure**. Because of the ambiguity of '*permissionless*', '*decentralized*', and '*trustless*', it is tempting to denounce these features as unrealistic, succumb to the pressure of scaling transaction throughput, and forego the initial vision. But similar to how no one entity controls the consensus mechanism, **no one person should be able to determine which tradeoffs we make when updating blockchain protocols**. With this in mind, the focus naturally shifts to reassessing how we govern blockchains themselves, an interesting variant of *[metagovernance](https://en.wiktionary.org/wiki/metagovernance)*.

In a [recent ZeroKnowledgeFM podcast](https://www.zeroknowledge.fm/52), Vlad Zamfir and Gavin Wood debate the merits of on-chain vs. off-chain governance. Gavin argues in favor of *on-chain governance*, which he defines as extending blockchain consensus technology to dictate not only what happens on the chain but also what happens with the chain. Vlad argues in opposition of formalized metagovernance and in support of *off-chain governance*. Under this model, informal discussions enable the community to signal preferences on software updates and come to *rough consensus* before making a decision.

At its core, we are asking the following question: *should we formalize blockchain governance with mechanisms similar to the ones used to reach consensus on transaction data or should we keep blockchain governance informal?* In this post, we will discuss the metagovernance debate in the hopes of clearly communicating each side; in the end, things are much more nuanced than a binary determination of right or wrong.

* [Off-Chain Governance via Rough Consensus](#rough)
* [Binding On-Chain Governance](#binding)

> This post contains a few claims that are not 100% true. For example, we can still *technically* execute hard forks on binding on-chain governance protocols, but I ignore such situations for the purpose of clearly communicating the main points...[sue me](https://www.youtube.com/watch?v=lppTJFYigoU)

## Off-Chain Governance via Rough Consensus <a name = "rough"></a>
> *Treat your articulation of governance problems and proposals as a loaded weapon and don't shoot in the dark.* ~ Vlad Zamfir, [Against on-chain governance](https://medium.com/@Vlad_Zamfir/against-on-chain-governance-a4ceacd040ca)

Before we jump into the metagovernance debate, we should consider the stakeholders involved in maintaining public blockchains. 

At the consensus level, **miners/validators** participate in an algorithm to organize transactions into blocks and commit blocks to the blockchain. Consensus algorithms implement an [anti-sybil](https://bitcoin.stackexchange.com/questions/50922/whats-a-sybil-attack) mechanism in order to attach a cost to the establishment of identity (corresponding to computational power in Proof of Work and coins staked in Proof of Stake). Indeed, the anti-sybil mechanism is a necessary part of any on-chain protocol because, in its absence, malicious actors could create an arbitrary number of identities to bias the outcome of a vote. 

In addition to miners/validators, there are other significant actors that support critical blockchain infrastucture. Some organizations and individuals run **full nodes** to enable secure user interaction with the blockchain. Using these full nodes as intermediaries, users may run **light clients**. Although light clients don't directly help the network, they provide a smooth end-user experience without incurring the overhead required to maintain a full node. Other blockchain stakeholders include DApp **developers**, DApp **users**, and even **coin hodlers**. For each of these stakeholder groups (with the exception of coin hodlers), there is no known anti-sybil mechanism to enable participation in on-chain protocols.

In the *rough consensus* model of off-chain governance, the community hosts regular developer, client, and research-specific video calls to gauge the sentiment of key stakeholders. Moderators such as Hudson Jameson are tasked with interpreting the lack of objection among participants to justify decisions made on behalf of the greater community. Although this style of rough consensus is far from optimal, it is arguably better than an on-chain protocol in which some stakeholders are entirely disenfranchised due to the lack of a viable anti-sybil mechanism. 

Under binding on-chain governance, full nodes automatically update when an on-chain process decides on an upgrade, regardless of whether their preferences were ultimately taken into account. As previously mentioned, full node operators are necessary supporters of blockchain infrastructure that deserve a voice in the metagovernance process. In the off-chain governance model, node operators that dissent with a protocol decision can avail themselves of the increased optionality provided by *[hard forks](https://medium.com/@Vlad_Zamfir/dear-ethereum-community-acfa99a037c4)*. Binding on-chain governance robs node operators of this choice by forcing them to follow on-chain processes in which their interests are not represented. This process is coercive, and "defaults are incredibly powerful" ([1](https://medium.com/@Vlad_Zamfir/against-on-chain-governance-a4ceacd040ca)).

Conversely, *rough consensus* is predicated on **thoughtful disagreement**; even in the midst of controversy, we can create open forums to communicate our thoughts and signal our preferences. **Just because we're making collective decisions does not necessarily mean that we need to agree on the criteria for making those decisions.** In the end, stakeholders are not affected by governance decisions in proportion to their coin holdings. With this in mind, we need to maintain informal off-chain governance processes to represent as many people as possible.

Looking forward, it is nearly impossible to predict the next major innovation. In the blockchain ecosystem, the pace of development moves considerably faster than any established policy-making process. Likewise, **when we formalize metagovernance based on how the blockchain works today, we tacitly strengthen and weaken certain power dynamics between existing stakeholders, thereby limiting the future participation of new stakeholders which may make the entire process more efficient.**

In summary, the main arguments for off-chain governance via *rough consensus* include:
1. Off-chain governance incorporates the preferences of stakeholders (i.e. node operators) for which there does not exist a known anti-sybil mechanism.
2. Binding on-chain governance is coercive; it forces full nodes to follow the outcome even if their preferences are not take into account.
3. **When we formalize governance prematurely, we disenfranchise future stakeholders and take away potentially more efficient, informal governance mechanisms that may arise organically from stakeholder interaction with the system.**
> Although the lack of an anti-sybil mechanism prevents the formalization of key stakeholder participation in governance, *rough consensus* through community discussions can offer an informal way for blockchain clients (and other important stakeholders) to voice their preferences.

## Binding On-Chain Governance <a name = "binding"></a>
> *Automatic upgrades are a sharp knife* ~ Gavin Wood, [Epicenter #259](https://www.youtube.com/watch?v=eP4mT19S_jg)

As we mentioned in the introduction, the value proposition of blockchains is the way by which consensus algorithms incentivize cooperation among a network's stakeholders and distribute trust. Rather than relying on a single person or organization, blockchains delegate the ordering of transactions to a group of validators/miners that follow a specific protocol for coming to agreement. This protocol is designed to foster an environment conducive to healthy competition so that users can trust and verify correct execution. 

The raison d'être of blockchains is to systematize this transaction ordering process. As Gavin mentions in [the debate](https://www.zeroknowledge.fm/52), "Our entire civilization is built on writing processes and adhering to them. Why? Because it means that the humans who are actually doing the mechanistic work of administering don't allow their own opinions to change the outcome." 

Although rough consensus sounds great *in theory*, delegating the interpretation of informal discussions to a small group of individuals is a recipe for disaster. As Hudson Jameson admits in [a less recent ZeroKnowledgeFM podcast](https://www.zeroknowledge.fm/43), "some voices are louder than others". Tasked with recording the opinions on developer calls, Hudson is in a unique position that probably should not exist. Even though we don't question his objectivity, if he was ever opinionated on a given issue, then he could miscommunicate or misdirect what actually transpired. As such, **the rough consensus model is susceptible to abuse; pleas to place trust in some abstract notion of *legitimacy* seem like a [red herring](https://en.wikipedia.org/wiki/Red_herring) from the stakeholders that have the most to lose.** 

Moreover, many arguments for off-chain governance misrepresent the optionality provided by the hard fork mechanism. Although binding on-chain governance may take away this path, it is worth considering whether the choice ever existed in the first place. It has been established that blockchains have strong network effects, which implies that only a few forks will ever be successful. Indeed, **off-chain governance proponents treat hard forks as if they are a bona fide mechanism for voicing dissent when, in reality, they often represent a false choice.** If a node operator disagrees with a protocol upgrade and is in the minority, standing by their principles is often not worth sacrificing the network effects enjoyed by staying a part of the existing community. If executing a hard fork means losing capital as well as future support from core and client developers, it seems obvious that the choice never really existed. Instead of pretending that hard forks offer users a voice, we need to accept the fact that informal governance is often just as coercive as mandatory software upgrades.

So we've thoroughly covered the flaws of off-chain governance, but **how do we make on-chain governance work?**

As we formalize the governance of these systems to mitigate the potential for abuse, it makes sense to vest power in the stakeholders who have *[skin in the game](https://en.wikipedia.org/wiki/Skin_in_the_Game_(book))*. For blockchain protocols, this stakeholder group is the coin hodlers. If coin hodlers make coercive upgrades that are disadvantageous to other stakeholders, we can expect the market to punish them via a decrease in the price of the native currency. Likewise, **instead of relying on the highly subjective interpretation of stakeholder opinions, we can use markets to hold coin hodlers accountable in their role as metagovernance agents.**

Admittedly, there are many problems with the naive implementation of on-chain governance via coin hodler voting, but we can mitigate these problems by incorporating additional checks and balances into our metagovernance system. Before mentioning specific proposals, it is necessary to emphasize the importance of experimenting with multiple on-chain governance protocols in parallel. Sequential testing via iterative development can be misleading because we'll cease experimentation prematurely. The [Hawthorne effect](https://en.wikipedia.org/wiki/Hawthorne_effect) reveals that the best approach is to test in parallel; in this context, that means deploying multiple on-chain governance protocols (on different blockchain networks) to determine what works and what doesn't.

In summary, the main arguments for *on-chain governance* include:
1. In practice, off-chain governance creates centralized points of trust that are susceptible to abuse.
2. It makes sense to vest responsibility for metagovernance decisions in the stakeholders that have *skin in the game* i.e. coin hodlers.
3. We can experiment with varying shades of liveness among metagovernance protocols to decide on the right balance; experimentation in parallel is the best approach!
4. We can mitigate the seemingly coercive nature of binding on-chain governance with additional protocols.

### Polkadot

* [Polkadot wiki: Governance](https://github.com/paritytech/polkadot/wiki/Governance)
* [How polkadot tackles the biggest problems facing blockchain innovators](https://medium.com/polkadot-network/how-polkadot-tackles-the-biggest-problems-facing-blockchain-innovators-1affc1309b0f)
* [Why on-chain governance](https://medium.com/polkadot-network/why-on-chain-governance-82ecf28f314c)

### Tezos

* [Towards Futarchy in Tezos](https://medium.com/tezos/towards-futarchy-in-tezos-54a7b8926967)
* ["There is no need for hard forks"](https://medium.com/tezos/there-is-no-need-for-hard-forks-86b68165e67d)
* [Hard Fork Politics](https://medium.com/tezos/hard-fork-politics-part-1-b767abbb5c19)

# Other References

* [Vlad and Gavin go head-to-head on blockchain governance](https://www.zeroknowledge.fm/52)
* [Blockchain Communities and Their Emergent Governance](https://medium.com/amentum/blockchain-communities-and-their-emergent-governance-cfe5627dcf52)

**Vlad Zamfir**
* [Blockchain Governance 101](https://blog.goodaudience.com/blockchain-governance-101-eea5201d7992)
* [My Intentions for Blockchain Governance](https://medium.com/@Vlad_Zamfir/my-intentions-for-blockchain-governance-801d19d378e5)
* [Against On-Chain Governance](https://medium.com/@Vlad_Zamfir/against-on-chain-governance-a4ceacd040ca)
* [Governance in Web2 vs Governance in Web3](https://www.youtube.com/watch?v=lLMVkmSTwho)

**Gavin Wood**
* [Epicenter: Substrate, Polkadot and the Case for On-Chain Governance](https://www.youtube.com/watch?v=eP4mT19S_jg)