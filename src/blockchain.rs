use crate::block;
use crate::address;

//this consists of a bunch of addresses, each of
//which contains a bunch of utxos, as many utxos can sit at the same addr

struct Blockchain {
    chain: Vec<Block>,
    mempool: Mempool,
}

impl Blockchain {
    fn new() -> Self {
        Self {
            chain: vec![],
            mempool: Mempool::new()
        }
    }
}