#![feature(async_await)]
use hidouki::{
    router::{JoinHandle, Route},
    Hidouki, Result,
};
use http::{Method, Request, Response};

fn main() {
    Hidouki::new("0.0.0.0:8080").routes(vec![Base]).launch();
}

struct Base;

impl Route for Base {
    const PATH: &'static str = "/";
    const METHOD: Method = Method::GET;
    fn route(_req: Request<String>) -> JoinHandle<Result<Response<String>>> {
        async_std::task::spawn(async { Ok(Response::new(String::new())) })
    }
}
