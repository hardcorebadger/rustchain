use std::fmt;

#[derive(Clone)]
pub struct Transaction {
	pub sender: String,
	pub recipient: String,
	pub amount: i64
}

// impl Display, get to_string for free
impl fmt::Display for Transaction {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "(from: {}, to: {}, amt: {}) ", self.sender, self.recipient, self.amount)
        }
}
