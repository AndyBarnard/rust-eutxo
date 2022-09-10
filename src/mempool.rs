use crate::transaction;

struct Mempool {
    txs: Vec<Transaction>
}

//TODO: convention around new() fxn naming...when to use new() and when to use a different name

impl Mempool {
    fn new() -> Self {
        Self {
            txs: vec![]
        }
    }

    fn add_tx(&mut self, tx: Transaction) {
        self.txs.push(tx);
    }

    fn contains_tx(&self, tx: Transaction) -> bool {
        self.txs.contains(tx)
    }

    fn clear_txs(&mut self) {
        self.txs.clear();
    }
}