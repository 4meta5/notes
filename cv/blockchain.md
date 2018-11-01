# Blockchain

* [How I got hooked](#beginning)
* [Research at UVA](#research)
* [Summer 2018](#summer2018)
* [Next Steps](#next)

> I actually lived another life in [finance](./markets.md) before blockchain. I believe that my experience studying markets has been invaluable in (1) helping me develop useful decision-making frameworks and (2) building an intuition with respect to incentive management.

## How I Got Hooked <a name="beginning"></a>
A few years ago, a good friend of mine told me that the next iteration of the internet was coming and it was going to be decentralized. At the time, we were both taking a class on internet protocols and had recently attended a lecture on the OSI model (a conceptual model for understanding the different layers for dot com internet protocols). A breakdown of the model revealed inefficiencies due to centralization (for example, the control of domain names), but we had been taught that these inefficiencies were necessary due to the economies of scale of technology companies. The possibility of a new system designed to solve these inefficiencies piqued my interest. After a few months of reading, I decided to fully commit myself to studying blockchain protocols. Since I went down the crypto rabbit hole, I have been impressed by the pace of progress in this space and the quality of output.


## Research at UVA <a name="research"></a>
Early in the summer of 2017, I reached out to Chris Dannen, founder of the hedge fund [Iterative Instinct](https://iterative.capital/), after reading his book [Introducing Ethereum and Solidity](https://www.apress.com/us/book/9781484225349). During the subsequent three months, Chris and Leo Zhang, Research Lead at Iterative, provided me with a high level introduction to the space. By the end of the summer, I was inspired to learn more about blockchain technology and apply myself to building software in the space.

With this in mind, I reached out to Professor [David Evans](http://www.cs.virginia.edu/~evans/) at the start of my 3rd year of college. Having previously taken Discrete Mathematics with Professor Evans and TAing for the class, I was familiar with his views on education and research.

> Professor Evans encourages intellectual curiosity and exploration outside of the classroom. He has consistently identified promising technology and, in doing so, has put UVA ahead of the curve. Two examples of this were his [Cryptocurrency course](http://bitcoin-class.org/) taught in 2014, and his [Operating Systems course](http://www.rust-class.org/) (the first taught in Rust) in Fall 2013.

Under Professor Evan's guidance, I received a rigorous introduction to cryptography and Proof of Work consensus in the Fall of 2017. The next semester I focused on layer 2 scaling solutions like payment channel networks and state channels. During that semester, Professor Evans voiced concerns over the viability of Ethereum's transition to Proof of Stake consensus. Although I was quite stubborn in my support of Ethereum at the time, I eventually came around to see the reasoning behind his doubts. 

> Even so, I have more confidence in the recent [ETH 2.0 spec](https://github.com/ethereum/eth2.0-specs/blob/master/specs/beacon-chain.md).

## Summer 2018 <a name="summer2018"></a>

In the summer of 2018, I undertook a graduate research internship at [Hyperledger](https://www.hyperledger.org/about), a subisidiary of the Linux foundation. My work aimed to optimize interaction with the blockchain by limiting invocation to dispute resolution. More specifically, my project focused on minimizing the involvement of litigation in the construction industry by tracking a contract's execution on a permissioned Hyperledger Fabric network. In the event of a dispute, the data from the Fabric network would be sent to an Ethereum smart contract for *trustless* dispute resolution (according to the conditions of the previously agreed upon construction contract). Although the project had lofty goals, it exposed me to the intricacies of utilizing the blockchain in the context of the law. Moreover, I gained experience coding in Golang and JavaScript, thereby developing a deeper appreciation for garbage collected languages.

Outside of my Hyperledger internship, I built blockchain developer courses for Blockmatics. In the previous semester, I had identified a growing opportunity in the developer onboarding space and was immediately drawn to the [Blockmatics](https://blockmatics.io/) team through interactions with Benjamin Mulkerin and Solomon Lederer. This experience helped me prepare to teach the UVA [CS1501 Introduction to Blockchain course](https://uvablockchain.gitbook.io/blockchain/) in the Fall of 2018. 

> resources for the class are organized in this github [repo](https://github.com/AmarRSingh/CS1501)

## Next Steps <a name="next"></a>
After starting to learn Rust this semester, I am committed to building incentive mechanisms motivated by [Radical Markets](https://press.princeton.edu/titles/11222.html) on Parity's [Substrate](https://github.com/paritytech/substrate) framework ([Polkadot](https://github.com/paritytech/polkadot) network). My next goal is to move to Berlin and help the Parity team build the next generation of blockchain software. 

I am also a part of the RadicalXChange movement led by Glen Weyl. My vision for a better society aligns with the ideals of this movement, and I hope to add value to the team in any way that I can.
