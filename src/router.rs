use crate::Result;
pub use async_std::task::JoinHandle;
use http::{Method, Request, Response};
use std::collections::HashMap;

pub(crate) struct Router {
    pub(crate) routes: HashMap<
        &'static str,
        HashMap<Method, RouteInternal>,
    >,
}

type RouteInternal = fn(Request<String>) -> JoinHandle<Result<Response<String>>>;

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn route<R: Route>(&mut self, _: R) {
        self.routes
            .entry(R::PATH)
            .and_modify(|map| {
                map.insert(R::METHOD, R::route);
            })
            .or_insert_with(|| {
                let mut map = HashMap::new();
                map.insert(
                    R::METHOD,
                    R::route as RouteInternal,
                );
                map
            });
    }
}

pub trait Route {
    const PATH: &'static str;
    const METHOD: Method;
    fn route(req: Request<String>) -> JoinHandle<Result<Response<String>>>;
}
