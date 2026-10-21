use crate::requests;
use crate::response;
use crate::response::{AddressMap, BoxedResponse, build_response};
use hyper::body::Incoming;
use hyper::http;
use hyper::service::Service;
use hyper::{Request, StatusCode, Uri, header};
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

        let host = match get_host(&req) {
            Some(uri) => uri,
            _ => {
                return Box::pin(async move {
                    requests::create_fallback_response(
                        &StatusCode::BAD_REQUEST,
                        &response::URI_FROM_REQUEST_ERROR,
                    )
                });
            }
        };

        // get target uri
        let address_params = match addresses.get(&host) {
            Some(params) => params.clone(),
            _ => {
                return Box::pin(async move {
                    requests::create_fallback_response(
                        &StatusCode::BAD_GATEWAY,
                        &response::URI_FROM_REQUEST_ERROR,
                    )
                });
            }
        };

        Box::pin(async move { build_response(req, address_params).await })
    }
}

pub fn get_host(req: &Request<Incoming>) -> Option<String> {
    // http2
    if let Some(host) = req.uri().host() {
        return Some(host.to_string());
    };

    // http 1
    let host_header = match req.headers().get(header::HOST) {
        Some(h) => h,
        _ => return None,
    };

    let host_str = match host_header.to_str() {
        Ok(hs) => hs,
        _ => return None,
    };

    let host_as_uri = match Uri::try_from(host_str) {
        Ok(hau) => hau,
        _ => return None,
    };

    if let Some(host) = host_as_uri.host() {
        return Some(host.to_string());
    }

    None
}
