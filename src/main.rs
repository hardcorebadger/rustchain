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
        router.get("/mine", move |r: &mut Request| {
                get_mine(r, Arc::clone(&CHAIN), node_id.clone()) }, "mine");
        router.get("/chain", move |r: &mut Request| {
                get_chain(r, Arc::clone(&CHAIN)) }, "chain");
        router.post("/transactions/new", move |r: &mut Request| {
                post_transaction(r, Arc::clone(&CHAIN)) }, "transaction");

        Iron::new(router).http("localhost:3000").unwrap();
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

        // parse new_block into response?

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut("Click. ** That's the sound of me mining a new block **\n");

	Ok(response)
}

fn get_chain(_request: &mut Request, _chain: Arc<RwLock<Blockchain>>) -> IronResult<Response> {
	let mut response = Response::new();
        let ch = _chain.read().unwrap();

        let full_chain = ch.get_chain();
        let chain_length = full_chain.len();

        // parse full_chain into response?
        // let json = serde_json::to_string(&somestructobject)?;

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
