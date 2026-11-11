use hyper::http::uri::InvalidUri;
use native_tls::Error as NativeTlsError;
use serde_json::Error as SerdeJsonError;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    Addresses(String),
    Config(String),
    Io(IoError),
    SerdeJson(SerdeJsonError),
    NativeTls(NativeTlsError),
    Uri(InvalidUri),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Addresses(err) => write!(f, "{}", err),
            Error::Config(err) => write!(f, "{}", err),
            Error::Io(err) => write!(f, "{}", err),
            Error::NativeTls(err) => write!(f, "{}", err),
            Error::SerdeJson(err) => write!(f, "{}", err),
            Error::Uri(err) => write!(f, "{}", err),
        }
    }
}
