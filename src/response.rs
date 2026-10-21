use crate::requests;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::body::Incoming;
use hyper::http::uri::InvalidUriParts;
use hyper::{Request, Response, StatusCode, Uri};
use std::collections::HashMap;

pub type BoxedResponse = Response<BoxBody<Bytes, hyper::Error>>;

pub const URI_FROM_REQUEST_ERROR: &str = "unable to parse upstream URI from request";
pub const UPSTREAM_URI_ERROR: &str = "falied to update request with upstream URI";

#[derive(Clone, Debug)]
pub struct AddressParams {
    pub uri: Uri,
    pub is_dangerous: bool,
}

pub type AddressMap = HashMap<String, AddressParams>;

fn update_request_with_dest_uri(
    mut req: Request<Incoming>,
    target_uri: &Uri,
) -> Result<Request<Incoming>, InvalidUriParts> {
    let mut dest_parts = target_uri.clone().into_parts();

    dest_parts.path_and_query = None;
    if let Some(path_and_query) = req.uri().path_and_query() {
        dest_parts.path_and_query = Some(path_and_query.clone());
    }

    *req.uri_mut() = match Uri::from_parts(dest_parts) {
        Ok(u) => u,
        Err(e) => return Err(e),
    };

    Ok(req)
}

pub async fn build_response(
    req: Request<Incoming>,
    address_params: AddressParams,
) -> Result<BoxedResponse, hyper::http::Error> {
    let version = req.version();

    let req = match update_request_with_dest_uri(req, &address_params.uri) {
        Ok(r) => r,
        _ => {
            return requests::create_fallback_response(
                &StatusCode::INTERNAL_SERVER_ERROR,
                &UPSTREAM_URI_ERROR,
            );
        }
    };

    let scheme = match req.uri().scheme() {
        Some(a) => a.as_str(),
        _ => "http",
    };

    match (version, scheme) {
        (hyper::Version::HTTP_2, "https") => {
            requests::send_http2_tls_request(req, address_params.is_dangerous).await
        }
        (hyper::Version::HTTP_2, _) => requests::send_http2_request(req).await,
        (_, "https") => requests::send_http1_tls_request(req, address_params.is_dangerous).await,
        _ => requests::send_http1_request(req).await,
    }
}
