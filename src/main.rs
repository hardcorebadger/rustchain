mod blockchain;
mod block;
mod transaction;

use blockchain::Blockchain;

fn main() {
    println!("Hello, world!");
    let mut chain = Blockchain::new();
    chain.new_transaction("me","you",5);
}