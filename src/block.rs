use transaction::Transaction;

pub struct Block {
	pub index: i64,
	pub timestamp: i64,
	pub transactions: Vec<Transaction>,
	pub proof: i64,
	pub previous_hash: String
}