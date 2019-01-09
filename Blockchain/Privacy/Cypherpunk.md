# Cypherpunk Reading List
> add notes from the book I'm working on -- I have a chapter on this

* Chaum 1983, 1990
* Finney 1993
* Medvinsky and Neuman 1993
* Szabo 1994, 1997
* Dai 1998
* Reagle 2005

* Nakamoto 2008

## Brief History of Digital Privacy
> *If privacy is outlawed, then only outlaws will have privacy.* - Philip R. Zimmermann, [Why I Wrote PGP](https://www.philzimmermann.com/EN/essays/WhyIWrotePGP.html)

Designed to empower individual rights, Pretty Good Privacy (PGP) was published on the Internet in 1991 by Phil Zimmermann. PGP is an encryption program that provides cryptographic privacy and authentication for data communication. By open sourcing PGP, Zimmermann violated the US export restrictions for cryptographic software, putting him at the center of a brutal criminal investigation that spanned nearly half a decade. In 1996, the US government dropped its case against Zimmermann, but the fight for privacy was far from over. As Zimmermann recounts in [Why I Wrote PGP](https://www.philzimmermann.com/EN/essays/WhyIWrotePGP.html), *"the only way to hold the line on privacy in the information age is strong cryptography"*.

Although the [struggle for control over digital interactions](https://en.wikipedia.org/wiki/Crypto_Wars#cite_note-55) [has](https://arstechnica.com/tech-policy/2015/01/uk-prime-minister-wants-backdoors-into-messaging-apps-or-hell-ban-them/) [not](https://blogs.wsj.com/digits/2015/01/16/obama-sides-with-cameron-in-encryption-fight/?guid=BL-DGB-39944&dsk=y) abated, progress on the implementation of cryptography has enabled the de facto encryption of communication over the Internet via SSL (Secure Socket Layer) and TLS (Transport Layer Security). When these protocols were first introduced, there was much of the same criticism that we see today in the form of "encryption enables criminal activity" and "you shouldn't be scared if you have nothing to hide". While these arguments may seem appealing on the surface, they ignore history by assuming **trust** in the underlying system and everyone else that uses it. 

This is a mistake. To foster an environment in which people can be truly safe, we must aspire towards systems in which *the requirement of trust itself is minimized*. Blockchain technology is a step in the right direction; it minimizes trust in the underlying system by incentivizing cooperation among a network's stakeholders [1]. Still, most contemporary blockchains operate as open public ledgers, displaying transaction data pseudonymously. This can lead to [critical leaks of user identity](https://www.technologyreview.com/s/608716/bitcoin-transactions-arent-as-anonymous-as-everyone-hoped/). To alleviate this problem and uphold digital privacy in all forms, it bears repeating the vision first set out by Eric Hughes in [A Cypherpunk Manifesto](https://www.activism.net/cypherpunk/manifesto.html): *"privacy in an open society requires anonymous transaction systems...An anonymous system empowers individuals to reveal their identity when desired and only when desired; this is the essence of privacy"*. 

[1] see explanations by (i) [Vitalik](https://blog.ethereum.org/2015/04/27/visions-part-2-the-problem-of-trust/) and (ii) [Preethi](https://medium.com/@preethikasireddy/eli5-what-do-we-mean-by-blockchains-are-trustless-aa420635d5f6) for a more nuanced explanation of how blockchains minimize trust


### More References
* [The Cyphernomicon](https://ia600208.us.archive.org/10/items/cyphernomicon/cyphernomicon.txt)
* [What is a Cypherpunk: CryptoAnarchy.wiki](https://cryptoanarchy.wiki/getting-started/what-is-a-cypherpunk)