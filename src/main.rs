extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate uuid;

use iron::prelude::*;
use iron::status;
use router::Router;
use uuid::Uuid;

mod blockchain;
mod block;
mod transaction;

use blockchain::Blockchain;

fn main() {
	let mut chain = Blockchain::new();
        println!("Serving on http://localhost:3000...");

        let mut router = Router::new();
        let node_id = Uuid::new_v4();

        router.get("/", get_hello, "root");
        router.get("/mine", get_mine, "mine");
        router.get("/chain", get_chain, "chain");
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

fn get_mine(_request: &mut Request) -> IronResult<Response> {
	let mut response = Response::new();

        //let last_block = chain.last_block().unwrap();
        //let last_proof = last_block.proof;
        //let new_proof = chain.proof_of_work(last_proof);

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
