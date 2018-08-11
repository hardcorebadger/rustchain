extern crate iron;
#[macro_use] extern crate mime;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

mod blockchain;
mod block;
mod transaction;

use blockchain::Blockchain;

fn main() {
	let mut chain = Blockchain::new();
  println!("Serving on http://localhost:3000...");
  
  let mut router = Router::new();

  router.get("/", get, "root");
  Iron::new(router).http("localhost:3000").unwrap();
    // chain.new_transaction("me","you",5);
}

fn get(_request: &mut Request) -> IronResult<Response> {
	let mut response = Response::new();
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut("hey!");

	Ok(response)
}