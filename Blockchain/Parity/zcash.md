# Motivating Construction of the ZCash Node

In a recent [blog post](https://www.parity.io/parity-teams-up-with-zcash-foundation-for-parity-zcash-client/), Parity announced a partnership with the Zcash foundation to build the first alternative client for Zcash in Rust. In this post, we'll motivate the project and call for support from the community.

> *In the end, I think applying these privacy and anonymity technologies to every blockchain out there is going to be essential because to limit freedom of expression, to limit freedom of association, you don't only have to deal with prior restraint. If you have the freedom to transact with everyone, but all your transactions are visible to everyone, then you can be punished after the fact for transacting with the wrong people, which essentially robs you of your freedom to transact. We have to realise that privacy is a foundational human right and without it, freedom of association, freedom of (political) expression, all go away.* - Andreas Antonopolous 

## Brief History of Digital Privacy
> *If privacy is outlawed, then only outlaws will have privacy.* - Philip R. Zimmermann, [Why I Wrote PGP](https://www.philzimmermann.com/EN/essays/WhyIWrotePGP.html)

Designed to empower individual rights, Pretty Good Privacy (PGP) was published on the Internet in 1991 by Phil Zimmermann. PGP is an encryption program that provides cryptographic privacy and authentication for data communication. By open sourcing PGP, Zimmermann violated the US export restrictions for cryptographic software, putting him at the center of a brutal criminal investigation that spanned nearly half a decade. In 1996, the US government dropped its case against Zimmermann, but the fight for privacy was far from over. As Zimmermann recounts in [Why I Wrote PGP](https://www.philzimmermann.com/EN/essays/WhyIWrotePGP.html), *"the only way to hold the line on privacy in the information age is strong cryptography"*.

Although the [struggle for control over digital interactions](https://en.wikipedia.org/wiki/Crypto_Wars#cite_note-55) [has](https://arstechnica.com/tech-policy/2015/01/uk-prime-minister-wants-backdoors-into-messaging-apps-or-hell-ban-them/) [not](https://blogs.wsj.com/digits/2015/01/16/obama-sides-with-cameron-in-encryption-fight/?guid=BL-DGB-39944&dsk=y) abated, progress on the implementation of Cryptography has enabled the de facto encryption of communication over the Internet via SSL (Secure Socket Layer) and TLS (Transport Layer Security). When these protocols were first introduced, there was much of the same criticism that we see today in the form of "encryption enables criminal activity" and "you shouldn't be scared if you have nothing to hide". While these arguments may seem appealing on the surface, they ignore history by assuming **trust** in the underlying system and everyone else that uses it. 

This is a fatal mistake. To build a digital environment in which people can be truly safe, we must aspire towards systems in which *the requirement of trust itself is minimized*. Blockchain technology is a step in the right direction; it minimizes trust in the underlying system by incentivizing cooperation among a network's stakeholders[1](#1). Still, most contemporary blockchains operate as open public ledgers, displaying transaction data pseudonanymously. This is very bad and can lead to [critical leaks of user identity](https://www.technologyreview.com/s/608716/bitcoin-transactions-arent-as-anonymous-as-everyone-hoped/). To alleviate this problem and uphold digital privacy in all forms, we take inspiration from [A Cypherpunk Manifesto](https://www.activism.net/cypherpunk/manifesto.html): *"privacy in an open society requires anonymous transaction systems...An anonymous system empowers individuals to reveal their identity when desired and only when desired; this is the essence of privacy"*. 

(1 <a name = "#1"></a>) see explanations by [Vitalik](https://blog.ethereum.org/2015/04/27/visions-part-2-the-problem-of-trust/) and [Preethi](https://medium.com/@preethikasireddy/eli5-what-do-we-mean-by-blockchains-are-trustless-aa420635d5f6) for a more nuanced explanation

## Zcash 



> *don't roll out your own crypto*

> it's naive to assume that there arent backdoors on other libraries

> for this reason, Parity is open sourcing all of its code and encouraging participating by the developer/cryptographer community. Many heads are better than one and we want to build tools that will help everyone.

> https://jeremykun.com/2014/02/08/introducing-elliptic-curves/

> https://jiggerwit.wordpress.com/2013/09/25/the-nsa-back-door-to-nist/

For this reason, I want to provide a few resources for zk-SNARKs to help people get started...I also want to provide some code resources for learning Rust. We need as many hands on deck as possible for this project.

> We also expect that when we launch Polkadot (LINK), encryption will play a role in number of different transaction systems

## zk-SNARKs


## zk-STARKs, Bulletproofs, and More Research


