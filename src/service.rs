use crate::response::{AddressMap, BoxedResponse, build_response};
use hyper::Request;
use hyper::body::Incoming;
use hyper::http;
use hyper::service::Service;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct Svc {
    pub addresses: Arc<AddressMap>,
}

impl Service<Request<Incoming>> for Svc {
    type Response = BoxedResponse;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let addresses = self.addresses.clone();

        // get address first then send to boxed response
        // instead of copying the entire address book

        // let host = match get_host(&req) {
        //     Some(uri) => uri,
        //     _ => {
        //         return requests::create_fallback_response(
        //             &StatusCode::BAD_REQUEST,
        //             &URI_FROM_REQUEST_ERROR,
        //         );
        //     }
        // };

        // // get target uri
        // let address_params = match addresses.get(&host) {
        //     Some(params) => params,
        //     _ => {
        //         return requests::create_fallback_response(
        //             &StatusCode::BAD_GATEWAY,
        //             &URI_FROM_REQUEST_ERROR,
        //         );
        //     }
        // };

        Box::pin(async move { build_response(req, addresses).await })
    }
}
