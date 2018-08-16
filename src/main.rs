#[macro_use] extern crate lazy_static;
#[macro_use] extern crate mime;
#[macro_use] extern crate serde_derive;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::{Arc, Mutex, RwLock};
use uuid::Uuid;

mod blockchain;
mod block;
mod transaction;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
	//let mut chain = Blockchain::new();
	//let mut chain = Arc::new(Mutex::new(Blockchain::new()));
        //static mut chain: Arc<RwLock<Blockchain>> = Arc::new(RwLock::new(Blockchain::new()));
        lazy_static! {
                static ref CHAIN: Arc<RwLock<Blockchain>> = {
                        let mut chain: Arc<RwLock<Blockchain>> = Arc::new(RwLock::new(Blockchain::new()));
                        chain
                };
        }

        println!("Serving on http://localhost:3000...");

        let mut router = Router::new();
        let node_id: Uuid = Uuid::new_v4();

        router.get("/", get_hello, "root");
//        let chain_ref_mine = Arc::clone(&chain);
        router.get("/mine", move |r: &mut Request| {
                get_mine(r, Arc::clone(&CHAIN), node_id.clone()) }, "mine");
//                get_mine(r, Arc::clone(&chain), &node_id) }, "mine");
        router.get("/chain", move |r: &mut Request| {
                //get_chain(r, &mut chain.read().unwrap()) }, "chain");
                get_chain(r, Arc::clone(&CHAIN)) }, "chain");
        router.post("/transactions/new", move |r: &mut Request| {
                //post_transaction(r, &mut chain.write().unwrap()) }, "transaction");
                post_transaction(r, Arc::clone(&CHAIN)) }, "transaction");

        Iron::new(router).http("localhost:3000").unwrap();
        // chain.new_transaction("me","you",5);
}

fn get_hello(_request: &mut Request) -> IronResult<Response> {
	let mut response = Response::new();
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut("Hey! I'm Rustchain, welcome to the future.\n");

	Ok(response)
}

fn get_mine(_request: &mut Request, _chain: Arc<RwLock<Blockchain>>,
                                    _node_id: Uuid) -> IronResult<Response> {
	let mut response = Response::new();
        let node_id_str = _node_id.simple().to_string();

        let mut ch = _chain.write().unwrap();
        let last_proof = ch.last_proof();
        let new_proof = ch.proof_of_work(last_proof);

        // index of block transaction will be put on
        let index = ch.new_transaction("0", node_id_str.as_str(), 1);
        // block transaction was put on
        let new_block = ch.new_block(new_proof);
        // should always be true
        //assert_eq!(index, new_block);
        assert_eq!(index, new_block.index);

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut("Click. ** That's the sound of me mining a new block **\n");

	Ok(response)
}

fn get_chain(_request: &mut Request, _chain: Arc<RwLock<Blockchain>>) -> IronResult<Response> {
	let mut response = Response::new();
        let ch = _chain.read().unwrap();

        //let full_chain = _chain.get_full_chain();
        //let chain_length = full_chain.length();

        // parse full_chain into response?

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut("Now listening to 'The Chain' by Fleetwood Mac.\n");

	Ok(response)
}

fn post_transaction(_request: &mut Request, _chain: Arc<RwLock<Blockchain>>) -> IronResult<Response> {
	let body = _request.get::<bodyparser::Raw>().unwrap().unwrap();
        let transaction: Transaction = serde_json::from_str(&body).unwrap();
        // transaction is now ready to use

        let mut ch = _chain.write().unwrap();

        // index of block this transaction will be on
        let index = ch.new_transaction(transaction.sender.as_str(), 
                                           transaction.recipient.as_str(),
                                           transaction.amount);

	let mut response = Response::new();
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut("hi\n");

	Ok(response)
}
