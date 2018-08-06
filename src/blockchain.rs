extern crate sha2;

use block::Block;
use transaction::Transaction;
use self::sha2::{Sha256, Digest};
use std::time;

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

	pub fn new_block(&mut self, new_proof: i64) {
                let index = (self.blocks.len()+1) as i64;
                let timestamp = match time::SystemTime::now()
                        .duration_since(time::UNIX_EPOCH) {
                        Ok(n) => n.as_secs() as i64,
                        _ => panic!("Invalid SystemTime"),
                };
                let transactions = self.current_transactions.clone();
                let proof = new_proof;
                let previous_hash = self.hash(self.last_block());

                self.current_transactions.clear();
                self.blocks.push( Block{index,timestamp,transactions,proof,previous_hash} );
	}
	pub fn new_transaction(&mut self, s:&str, r:&str, amount:i64) {
		let sender = s.to_owned();
		let recipient = r.to_owned();
		self.current_transactions.push(Transaction{sender,recipient,amount});
	}

	pub fn last_block(&self) -> Option<&Block> {
                self.blocks.last()
	}

	pub fn hash(&self, block: Option<&Block>) -> String {
                match block {
                        Some(b) => {
                                let mut hasher = Sha256::default();
                                hasher.input(b.to_string().as_bytes());
                                String::from_utf8(hasher.result().as_slice().to_vec()).unwrap()
                        },
                        None => panic!("Invalid block passed to hash")
                }
	}
}
