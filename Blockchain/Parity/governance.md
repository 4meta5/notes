# Automatic Upgrades Are A Sharp Knife

Blockchain protocols leverage a combination of cryptography, game theory, and network security to reach consensus on the ordering of transactions in a distributed setting. Likewise, the value proposition of blockchain technology lies in its ability to incentivize cooperation among a network's stakeholders, even if the set of stakeholders isn't well-defined. By fostering a *permissionless* miner/validator set, blockchains can achieve decentralization, a necessary prerequisite for adequately distributing trust to claim *trustlessness*.

Admittedly, many of these properties are hard to define and even more difficult to measure. Because of the ambiguity of '*permissionless*', '*decentralized*', and '*trustless*', it is tempting to denounce these features as unrealistic, succumb to the pressure of scaling transaction throughput, and forego the initial vision. But similar to how no one entity controls the blockchain consensus mechanism, no one person should be able to determine which tradeoffs we make when updating blockchain protocols. With this in mind, the focus naturally shifts to reassessing how we govern blockchains themselves, a nuanced sort of *[metagovernance](https://en.wiktionary.org/wiki/metagovernance)*.



In a [recent ZeroKnowledgeFM podcast](https://www.zeroknowledge.fm/52), Vlad Zamfir and Gavin Wood debate the merits of on-chain vs. off-chain governance. Gavin argues in favor of *on-chain governance*, which consists of extending blockchain consensus technology to dictate not only what happens on the chain but also what happens with the chain. Vlad 

At this point in time, the community seems split in terms of the best way to proceed; should we formalize governance of the blockchain by using the same mechanisms we use to reach consensus on transaction data or should we keep blockchain governance informal to keep the process as open and dynamic as possible?

Blockchain governance is a unique form of *[metagovernance](https://en.wiktionary.org/wiki/metagovernance)*. 

*On-chain governance* consists of extending blockchain consensus technology to dictate not only what happens on the chain but also what happens with the chain. 


On the o†her hand, *off-chain governance* prefers a more informal route in which discussions enable the *community* to come to consensus on software updates.

-- where do I want to define stakeholders...full node operators, light clients, community members, dapp developers, coin holders, etc.

## Off-Chain Governance via Rough Consensus

It is nearly impossible to predict the next major innovation. In the blockchain ecosystem, the pace of development moves considerably faster than any established policy-making process. Likewise, when we formalize metagovernance based on how the blockchain works today, we tacitly strengthen and weaken certain power dynamics between existing stakeholders, thereby limiting the future participation of new stakeholders which may make the entire process more efficient.

When we formalize governance prematurely, we disenfranchise future stakeholders and take away potentially more efficient, informal governance mechanisms that may arise organically from stakeholder interaction with the system (blockchain in this case).

One approach is to make the policy-making process more dynamic in order to keep up with innovation; another is to make governance informal so that it can adapt to innovation as it occurs.
 

— consider the limitations of coinholder voting for blockchain governance decisions.
——Parity Light Protocol is designed to incorporate light clients and may entail incentivizing full nodes via micropayment models. Parity’s substrate protocol also utilizes Collators in addition to Validators; these collators are also tasked as fishermen. In the future, we don’t know what kinds of actors may surface, and, if we restrict voice in the context of governance to coinholders, then we preemptively disenfranchise future stakeholder groups, thereby limiting what we can achieve in the future. The best way forward is to try and expand our options — this is also the best strategy in chess.

- contentious hard forks are more common; but 'these aren't bad'

**Against On-Chain Governance**<br>
- "On-chain governance" refers to the idea that the blockchain nodes automatically upgrade when an on-chain governance process decides on an upgrade and that it's time to install it. No hard forks required.
- Adopting on-chain governance is incredibly risky because it always represents a revolution; a revoluton that overthrows the processes that govern full nodes
- With off-chain governance (the current norm), a node operator has to consciously decide whether to install a hard fork to have his node be consensus-compatible with the nodes of operators who also decided to install that hard fork.
- On-chain governance makes node operator participation in governance completely unnecessary. It makes it so that a node operator, making no decision, follows the decisions made by the on-chain process. Defaults are incredibly powerful: The more nodes follow the default, the less feasible it is for a concerned node operator to refuse to install a hard fork.
- node operators (and therefore users) will necessarily be robbed of their participation in governance, by any on-chain governance proposal; the role of full nodes in off-chain governance provides an important check to balance against the power of processes that makes changes to software...On-chain governance removes that check, and that balance it provides.
- Unless there are governance processes that get Sybil-resistant input from node operators, on-chain governance therefore always has the potential to disenfranchise node operators (and users) of the blockchain.
- Coin holder interest and user interests are not naturally aligned.
    - **the market between blockchains is absolutely not perfectly competitive. It is highly oligopolistic because blockchains have strong network effects**
- If we find that we can build useful blockchain governance tools using the blockchain, that's great! However, overthrowing the processes that govern the software implementations of the blockchain, or the processes that govern full nodes is most probably not well-advised.

> *Treat your articulation of governance problems and proposals as a loaded weapon and don't shoot in the dark.*

basic idea: just because the process is informal and NOBODY understands it doesn't mean that it doesn't work...this is a red herring for sure

## Binding On-Chain Governance

In *Cypherpunks*, Jeremie Zimmerman argues that "policy has to adapt to society, and not the other way around". He expands on his point, claiming that "good policy looks at the world and adapts to it in order to correct what is wrong and enable what is good. I'm convinced that when you enable the most powerful industrial actors to decide what policy should be, you don't go that way".


> Can we make governance dynamic such that we formalize governance, while still enabling the changes necessary to progress and update in a smooth way? 

> "any governance that depends on high turnout is doomed to fail" - Fredrik

Discussion of adaptive quorum biasing -- pros and cons?

One counterargument is that current stakeholders rarely have the incentive to cede power or change the existing power structure.

-- Hudson Jameson: some voices are louder than others; he is tasked as the interpreter

## WHO IS RIGHT?

The premise is incorrect here!

— there’s a balance here that needs to be considered, and this balance is important. Goldilocks principle...in my opinion, the best approach will be a diversity of networks with different shades of formalized governance. See McKie and Pruitt recent blog post.

*With off-chain governance (the current norm), a node operator has to consciously decide whether to install a hard fork to have his node be consensus-compatible with the nodes of operators who also decided to install that hard fork.*

--> is this even correct; I guess the legitimate question would be whether the hard fork is opt in or opt out. If it is opt in, then it seems they have a choice in which the default is to continue on their path and the opt in decision allows them to voice their support of the hard fork; but the DAO hard fork was opt out for most clients and, indeed, the organizations behind the main clients revealed that they would not continue updating the software for the other fork. When your decision is made for you, do you really have a decision?

-- analogy:  

*node operators (and therefore users) will necessarily be robbed of their participation in governance, by any on-chain governance proposal*

- can you be robbed of something you never had?

- contentious hard forks are misleading; do people really have a choice? Not really

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