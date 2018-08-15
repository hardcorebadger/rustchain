extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate uuid;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

mod blockchain;
mod block;
mod transaction;

use blockchain::Blockchain;

fn main() {
	//let mut chain = Blockchain::new();
	let mut chain = Arc::new(Mutex::new(Blockchain::new()));
        println!("Serving on http://localhost:3000...");

        let mut router = Router::new();
        let node_id = Uuid::new_v4();

        router.get("/", get_hello, "root");
        //router.get("/mine", get_mine, "mine");
        router.get("/mine", move |r: &mut Request| {
                get_mine(r, &mut chain.lock().unwrap(), &node_id) }, "mine");
        //router.get("/chain", get_chain, "chain");
        router.get("/chain", get_chain, "chain");
        //router.post("/transactions/new", post_transaction, "transaction");
        router.post("/transactions/new", post_transaction, "transaction");
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

fn get_mine(_request: &mut Request, _chain: &mut Blockchain, _node_id: &Uuid) -> IronResult<Response> {
	let mut response = Response::new();
        let node_id_str = _node_id.simple().to_string();

        let last_proof = _chain.last_proof();
        let new_proof = _chain.proof_of_work(last_proof);

        _chain.new_transaction("0", node_id_str.as_str(), 1);

        let new_block = _chain.new_block(new_proof);

        //parse new_block JSON

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut("Click. ** That's the sound of me mining a new block **\n");

	Ok(response)
}

fn get_chain(_request: &mut Request) -> IronResult<Response> {
	let mut response = Response::new();
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut("Now listening to 'The Chain' by Fleetwood Mac.\n");

	Ok(response)
}

fn post_transaction(_request: &mut Request) -> IronResult<Response> {
	let mut response = Response::new();
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut("Initializing transaction. on. the chain.\n");

	Ok(response)
}
//}
