use router::Router;
use iron::{Iron, Response, status, Request};

fn main() {
    let mut router = Router::new();

    router.get("/test/*", |_: &mut Request| Ok(Response::with((status::Ok, "Hi"))), "1");

    Iron::new(router).http("localhost:3333").unwrap();
}


// This works, so why does it not work for docs.rs ????