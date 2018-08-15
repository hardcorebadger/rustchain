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
		let mut blocks = Vec::new();
		let current_transactions = Vec::new();

                // Manually create genesis block
                let timestamp = match time::SystemTime::now()
                        .duration_since(time::UNIX_EPOCH) {
                        Ok(n) => n.as_secs() as i64,
                        _ => panic!("Invalid SystemTime"),
                };
                let transactions = current_transactions.clone();
                let previous_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();

                // Add genesis block as first block
                blocks.push(Block{index:1, timestamp, transactions, proof:100, previous_hash});

		Blockchain{blocks,current_transactions}
	}

	pub fn new_block(&mut self, new_proof: i64) {
                // Create a new block in the blockchain
                let index = (self.blocks.len()+1) as i64;
                let timestamp = match time::SystemTime::now()
                        .duration_since(time::UNIX_EPOCH) {
                        Ok(n) => n.as_secs() as i64,
                        _ => panic!("Invalid SystemTime"),
                };
                let transactions = self.current_transactions.clone();
                let proof = new_proof;
                let previous_hash = self.hash_last();

                self.current_transactions.clear();
                self.blocks.push( Block{index,timestamp,transactions,proof,previous_hash});
	}

	pub fn new_transaction(&mut self, s:&str, r:&str, amount:i64) {
                // Create a new transaction to go into the next mined block
		let sender = s.to_owned();
		let recipient = r.to_owned();
		self.current_transactions.push(Transaction{sender,recipient,amount});
	}

	pub fn last_block(&mut self) -> Option<&Block> {
                self.blocks.last()
	}

        pub fn last_proof(&mut self) -> i64 {
                self.blocks.last().unwrap().proof
        }

        fn hash_last(&mut self) -> String {
                self.hash(self.blocks.last())
        }

	pub fn hash(&self, block: Option<&Block>) -> String {
                // Creates a SHA-256 hash of a block
                match block {
                        Some(b) => {
                                let mut hasher = Sha256::default();
                                hasher.input(b.to_string().as_bytes());
                                String::from_utf8(hasher.result().as_slice().to_vec()).unwrap()
                        },
                        None => panic!("Invalid block passed to hash")
                }
	}

        pub fn proof_of_work(&self, last_proof: i64) -> i64 {
                // Simple proof of work algorithm:
                //   find a number p such that hash(last_proof, p) 
                //   contains 4 leading zeros
                let mut p: i64 = 0;
                while !self.valid_proof(last_proof, p) { p += 1; }
                p
        }

        fn valid_proof(&self, last_proof: i64, new_proof: i64) -> bool{
                // Validate the proof: Does hash(last_proof,new_proof)
                // contain 4 leading zeros
                let mut input_str = String::new();
                input_str.push_str(last_proof.to_string().as_str());
                input_str.push_str(new_proof.to_string().as_str());

                let mut hasher = Sha256::default();
                hasher.input(input_str.as_bytes());

                String::from_utf8(hasher.result().as_slice().to_vec())
                        .unwrap().starts_with("0000")
        }
}
