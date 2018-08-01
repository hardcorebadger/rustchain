use block::Block;
use transaction::Transaction;

pub struct Blockchain {
	blocks: Vec<Block>,
	current_transactions: Vec<Transaction>
}

impl Blockchain {

	pub fn new() -> Blockchain {
		let blocks = Vec::new();
		let current_transactions = Vec::new();
		Blockchain{blocks,current_transactions}
	}

	// pub fn new_block(&self) {

	// }
	pub fn new_transaction(&mut self, s:&str, r:&str, amount:i64) {
		let sender = s.to_owned();
		let recipient = r.to_owned();
		self.current_transactions.push(Transaction{sender,recipient,amount});
	}

	// pub fn last_block(&self) -> Block {
	// }

	// pub fn hash(&self, block: Block) {

	// }
}