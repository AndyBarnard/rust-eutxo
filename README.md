# This is an extended UTxO proof-of-stake L1 implemented in Rust. 
Current state of this project is: major WIP and doesn't yet compile.
Have taken some inspiration from Cardano/Plutus in this implementation. 
At the moment I'm still sketching out the design.

## A brief explanation of the extended UTxO model
To understand the extended UTxO (eUTxO) model, we need to understand the UTxO model.
UTxO stands for Unspend Transaction Output, and was pioneered by Satoshi N. in the Bitcoin whitepaper. This model is analogous to the way we use cash, where we might have a $20 bill, a $5 bill, etc. When you pay for things with cash like this, the transaction fully consumes the bills you use to pay and you receive change in some other denominations. But you wouldn't pay for something with a $20 bill and the cashier rips off a piece of that bill and gives it back to you. In this model, transactions are functions that fully consume their input utxos. By contrast, the account model that most L1s use, is simply an address with a balance. With Ethereum, for example, If I have 100 ETH in my wallet and send you 3 ETH + fees, now I have 97 ETH. This is analogous to having a "100 ETH bill" and ripping off a piece of that bill to send you money. This account model is quite intuitive for most people, as it's not dissimilar to a bank account where it's simply a number (e.g. $12.07, if you were curious of my current account balance).

In the eUTxO model, we extend the UTxO model to add an arbitrary piece of data, called the "datum" which sits at the UTxO at a given address. Additionally, we have a validator script which can also be thought of as a smart contract, though it's just a boolean function. In contrast to most smart contracts, e.g. those written on Ethereum, eUTxO smart contracts
consist only of validation logic, whereas Ethereum contracts contain both validation logic and execution logic. In our model, however, execution logic is created off-chain by the user's wallet. The wallet can create whatever transaction it likes, and all we care about is that it passes our boolean function. 

## Why use this model?
If you have experience developing on both an account-model L1 and an eUTxO L1, you should immediately notice a difference in the way you have to think to solve problems. In the eUTxO model, all transactions explicitly state their dependencies (input utxos). This causes a big shift in the way we think about smart contract development. So what's the point?

For one, let's look at one of the most talked about topics in blockchain in recent years: scalability. Let's take a look at how transactions are collated into blocks in an account-based L1 (e.g. Ethereum, Fantom, Solana). When the EVM executes the bytecode for all the transactions in a block, it starts with the first transaction. This tx has write access to the entire state of the blockchain, and as a result, all other transactions must wait their turn to execute. If transactions were executed concurrently in the account model, race conditions woule be inevitable, as multiple transactions have mutable references to the same state. So this model is inherently serial, and does not scale well at all. 

In the eUTxO model, by contrast, every transaction explicitly states its inputs, meaning that concurrent execution of transactions is sort of "baked in" at the lowest level. E.g. if I send money from my wallet to my grandma's wallet, there's no reason that transaction should interfere with your transaction, where you send money from your wallet to some defi app. Each of these transactions rely on different pieces of the same state, rather than the entire same state. 

## Why use Rust?
Rust's strong type system and support for functional programming make it a great choice for implementing an eUTxO L1. Compared to writing Haskell code for Cardano, the Haskell experience is full of language extensions, template haskell, and other boilerplate that makes it kind of a pain. Regardless, blockchains do appear to be a functional programming problem. 

## Why build this? It seems like a ton of work
I think I need to implement an L1 myself to fully understand how blockchains work. Reading and dapp development aren't enough.

