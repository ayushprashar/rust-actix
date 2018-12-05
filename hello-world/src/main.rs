extern crate actix_web;

use actix_web::{ server, App, HttpRequest };

fn index(req: &HttpRequest) -> &'static str {
	"Ayush Prashar"
}
fn main() {
    server::new(
    		|| App::new()
    			.resource("/", |r| r.f(index)))
    			.bind("127.0.0.1:8088")
    			.unwrap()
    			.run();
    		
}
