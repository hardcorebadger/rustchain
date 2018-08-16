use std::fmt;
use transaction::Transaction;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
	pub index: i64,
	pub timestamp: i64,
	pub transactions: Vec<Transaction>,
	pub proof: i64,
	pub previous_hash: String
}

impl Block {
        fn transaction_string(&self) -> String {
                let mut trans_string = String::new();
                for t in self.transactions.iter() {
                        trans_string.push_str(&t.to_string())
                }
                trans_string
        }
}

// impl Display, get to_string for free
impl fmt::Display for Block {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{{ index: {} \ntimestamp: {} \ntransactions: {} \nproof: {} \nprevious_hash: {} }}\n", self.index, self.timestamp, self.transaction_string(), self.proof, self.previous_hash)
        }
}
