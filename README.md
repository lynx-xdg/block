# Block

This is a simple blockchain implementation in rust. Disclaimer: this project is not related to cryptocurrency, it uses similar technology, but not for creating virtual currency.

This project uses blockchain technology in order to store information in a way that modifing past information becomes impossible.

## Parts of a blockchain

### Genesis block

The genesis block is special because it contains no information about previous blocks, since it's supposed to be used as the first block in a blockchain. Usually this block does not contain any actual information. It does share the same struct type with a regular block, the difference is that this block contains values that are randomly assigned by the creator.

### Blocks

A block is a data structure that contains an `id`, a `timestamp`, a hash of the previos block, some data to be stored, a `nonce` and a `difficulty`.

#### id

The id is like an index into the blockchain, it starts at zero and counts up following the chain. Every id is unique.

#### timestamp

The timestamp is used for deciding the difficulty of the next block.

#### nonce

The nonce is a randomly generated value added to the block in order to change the hash so the difficulty requirements are met.

#### difficulty

The difficulty is an optional system in blockchains that works as a proof of work. This prevents from spamming blocks onto the chain by requiring some computational effort be put into generating a block. In this implementation the difficulty dynamically changes in order to force the time between blocks mined to be around `1000ms`

## Mining

Mining is finding a `nonce` value that changes the hash of a block in a way that makes the hexadecimal value of the hash smaller than a number decided by the difficulty. Because the difficulty adapts to the mining speed. Even tough the mining speed might be incredibly high, the rate of blocks being attached will stay around the above `1000ms`

## Possible further experiments

 - writing a multithreaded miner that actually works
 - adding networking and serialisation so that the blockchain can become decentralised
