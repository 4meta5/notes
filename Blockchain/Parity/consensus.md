# Substrate Consensus Modules

## GRANDPA (SHAFT)
* SHAFT: Shared Ancestry Finality Tool
* GRANDPA

> iteration or two beyond Rhododendron; progressive finality

> hybrid pbft/aurand


**Aurand for Growth & PBFT for Finality**
* Aurand provides *predictable chain growth* under weighted validator set
    * authorities from the validator set that are randomly chosen; they issue blocks
* Well-justified blocks selected for decentralized finality under PBFT-like protocol
    * use probabilistic finality as a decentralized broadcast mechanism for blocks that are sufficiently old; we hold some vote to achieve full finality

### Parity / Polkadot

## Aura
> probabilistic finality (poa)

### Aurand
> **Ouroboros**-variant

## Rhododendron
> BFT-eque; pulls from PBFT

* instant finality

## Shasper
> based on Serenity (Eth 2.0)