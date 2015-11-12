use std::result;
use std::fmt;

use std::io;
use rustc_serialize::json;
use hyper;
use std::string::FromUtf8Error;
use std::num::ParseIntError;

pub type Result<T> = result::Result<T, ConnectError>;

#[derive(Debug)]
pub enum ConnectError {
  JSONError(json::DecoderError),
  IOError(io::Error),
  //URLError(url::parser::ParseError),
  HTTPError(hyper::error::Error),
  Utf8Error(FromUtf8Error),
  ParseIntError(ParseIntError)
}

impl From<json::DecoderError> for ConnectError {
  fn from(err: json::DecoderError) -> ConnectError {
    ConnectError::JSONError(err)
  }
}

//impl From<url::parser::ParseError> for ConnectError {
  //fn from(err: url::parser::ParseError) -> ConnectError {
    //ConnectError::URLError(err)
  //}
//}

impl From<io::Error> for ConnectError {
  fn from(err: io::Error) -> ConnectError {
    ConnectError::IOError(err)
  }
}

impl From<hyper::error::Error> for ConnectError {
  fn from(err: hyper::error::Error) -> ConnectError {
    ConnectError::HTTPError(err)
  }
}

impl From<FromUtf8Error> for ConnectError {
  fn from(err: FromUtf8Error) -> ConnectError {
    ConnectError::Utf8Error(err)
  }
}

impl From<ParseIntError> for ConnectError {
  fn from(err: ParseIntError) -> ConnectError {
    ConnectError::ParseIntError(err)
  }
}

impl fmt::Display for ConnectError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ConnectError::JSONError(ref error) => write!(f, "JSON Error: {}", error),
      ConnectError::IOError(ref error) => write!(f, "IO Error: {}", error),
      ConnectError::HTTPError(ref error) => write!(f, "HTTPError: {}", error),
      ConnectError::Utf8Error(ref error) => write!(f, "Utf8Error: {}", error),
      ConnectError::ParseIntError(ref error) => write!(f, "ParseIntError: {}", error)
    }
  }
}
