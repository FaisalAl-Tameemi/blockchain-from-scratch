use crate::Transaction;

#[derive(Debug, Clone)]
pub struct TransactionPool(pub Vec<Transaction>);

impl TransactionPool {
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.0.push(transaction);
    }
}