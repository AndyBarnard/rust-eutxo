# This is a brief explanation of the source code. WIP

## Blockchain
A chain of blocks

## Block
A bunch of transactions to be executed CONCURRENTLY (aw yeahhh) as well as the merkle root, prev_block_hash, block_timestamp, etc....

## Transaction
A transaction is just a function that consumes at least 1 input UTxO, and outputs at least 1 output UTxO. Before being added to a block, a Transaction goes to the Mempool.

## Mempool
Simple implementation of a mempool. Basically just a COD lobby for executable bytecode.

## Utxo
A Utxo contains either a pubkeyhash or a field for any type that implements the trait `Validates`. Why make this polymorphic? Why not simply implement the `validate` function for the `Utxo` type? The answer is that the `validate` function is itself the smart contract (i.e. executable code). When constructing a `Utxo`, we don't know in advance what smart contract will lock this utxo, so we say anything that implements the `validate` function is sufficient. 

## Smart contracts
A smart contract is simply a boolean function called `validate` that is a field on the `Utxo` type. You can think of a smart contract as a "lock" on a utxo that only allows this utxo to be consumed if `validate` returns `true`.

## Public Key Hash (PubKeyHash or pkh)
The other form of "lock" for a utxo is a pubkeyhash, which allows the utxo to be consumed when signed by the right private key. In this case, the field `utxo_lock` won't have a validator function, but rather a pkh.




