#![feature(async_await)]
use hidouki::{router::route, Hidouki};

fn main() {
    Hidouki::new("0.0.0.0:8080").routes(vec![base]).launch();
}

#[route(GET "/hello/there")]
async fn base(req: Request<String>) -> Result<Response<String>> {
    Ok(Response::new(String::new()))
}

// This expands out to this:
// #[allow(non_camel_case_types)]
// struct base;
// impl hidouki::router::Route for base {
//     const PATH: &'static str = "/hello/there";
//     const METHOD: hidouki::Method = hidouki::Method::GET;
//     fn route (
//         req: hidouki::Request<String>
//     ) -> hidouki::router::JoinHandle<hidouki::Result<hidouki::Response<String>>> {
//         use hidouki::{ Response, Request };
//         hidouki::spawn(async { Ok(Response::new(String::new()))})
//     }
// }
//
// While you can write it yourself, it's highly discouraged.
