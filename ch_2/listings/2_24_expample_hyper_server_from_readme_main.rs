extern crate hyper;                                      //#A

use hyper::Server;                                       //#B
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

fn hello(_: Request, res: Response<Fresh>) {             //#C
    res.send(b"Hello World!").unwrap();
}

fn main() {
    Server::http("127.0.0.1:3000").unwrap().handle(hello);
}
// #A Import an external crate into a file
// #B Imports specific parts of the hyper crate into scope
// #C Using _ for the param that we are not using
