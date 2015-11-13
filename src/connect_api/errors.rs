use std::result;
use std::fmt;
use std::error;

use std::io;
use rustc_serialize::json;
use hyper;
use std::string::FromUtf8Error;
use std::num::ParseIntError;

pub type Result<T> = result::Result<T, ConnectError>;

#[derive(Debug)]
pub enum ConnectError {
  JSONDecodeError(json::DecoderError),
  JSONEncodeError(json::EncoderError),
  IOError(io::Error),
  //URLError(url::parser::ParseError),
  HTTPError(hyper::error::Error),
  Utf8Error(FromUtf8Error),
  ParseIntError(ParseIntError)
}

macro_rules! error_mapping {
  ($from:ty, $to:expr) => (
    impl From<$from> for ConnectError {
      fn from(err: $from) -> ConnectError {
        $to(err)
      }
    }
  );
}

error_mapping!(json::DecoderError, ConnectError::JSONDecodeError);
error_mapping!(json::EncoderError, ConnectError::JSONEncodeError);
error_mapping!(io::Error, ConnectError::IOError);
error_mapping!(hyper::error::Error, ConnectError::HTTPError);
error_mapping!(FromUtf8Error, ConnectError::Utf8Error);
error_mapping!(ParseIntError, ConnectError::ParseIntError);

//impl From<url::parser::ParseError> for ConnectError {
  //fn from(err: url::parser::ParseError) -> ConnectError {
    //ConnectError::URLError(err)
  //}
//}

macro_rules! map_descriptions {
  ( $target:expr,
    $($x:path),*
  ) => {
    match $target {
      $( $x(ref error) => error.description() ),*
    }
  }
}

impl error::Error for ConnectError {
  fn description(&self) -> &str {
    map_descriptions!(
      *self,
      ConnectError::JSONDecodeError, ConnectError::JSONEncodeError, ConnectError::IOError,
      ConnectError::HTTPError, ConnectError::Utf8Error, ConnectError::ParseIntError
    )
  }
}

impl fmt::Display for ConnectError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ConnectError::JSONDecodeError(ref error) => write!(f, "JSON Decode Error: {}", error),
      ConnectError::JSONEncodeError(ref error) => write!(f, "JSON Encode Error: {}", error),
      ConnectError::IOError(ref error) => write!(f, "IO Error: {}", error),
      ConnectError::HTTPError(ref error) => write!(f, "HTTPError: {}", error),
      ConnectError::Utf8Error(ref error) => write!(f, "Utf8Error: {}", error),
      ConnectError::ParseIntError(ref error) => write!(f, "ParseIntError: {}", error)
    }
  }
}
