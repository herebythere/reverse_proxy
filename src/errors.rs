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
            Error::Addresses(err) => write!(f, "[Error::Addresses] {}", err),
            Error::Config(err) => write!(f, "[Error::Config] {}", err),
            Error::Io(err) => write!(f, "[Error::Io] {}", err),
            Error::NativeTls(err) => write!(f, "[Error::Native_tls] {}", err),
            Error::SerdeJson(err) => write!(f, "[Error::Serde_json] {}", err),
            Error::Uri(err) => write!(f, "[Error::Uri] {}", err),
        }
    }
}
