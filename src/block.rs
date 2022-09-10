use crate::transaction;

use anyhow::Error;

mod block {

    struct Block {
        id: u32,
        prev_hash: String,
        merkle_root: String,
        txs: Vec<Transaction>,
        timestamp: u32,
        difficulty: u32,
    }

    //TODO: maybe don't need to pass the timestamp as a param, maybe in new() it can query a global variable, etc.

    impl Block {
        fn new(
            id: u32,
            prev_hash: String,
            merkle_root: String,
            txs: Vec<Transaction>,
            timestamp: u32,
            difficulty: u32,
        ) -> Self {
            Self {
                id,
                prev_hash,
                merkle_root,
                txs,
                timestamp,
                difficulty,
            }
        }

        fn create_genesis_block() -> Self {
            Self {
                id: 0,
                prev_hash: "",
                merkle_root: "",
                txs: vec![],
                timestamp: 0,
                difficulty: 0,
            }
        }

        fn mine_block() -> anyhow::Result<Block> {
            let test_str = String::from("test string");

            if consensus(&test_str) {
                Ok(Block)
            }

            Error("didnt pass consensus")
        }
    }
}
