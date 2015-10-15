use std::fs::File;
use std::io::Error;
use std::io::{Read, Write};

#[derive(RustcDecodable, Debug)]
pub struct SystemCredentials {
  login: String,
  password: String
}

use std::fs;
pub fn read_scc_credentials() -> Result<SystemCredentials, Error> {
    let mut file = try!(File::open("/etc/zypp/credentials.d/SCCcredentials"));

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    debug!("Loaded existing SCC credentials {:?}", buffer);

    let mut shards = buffer.trim().split(":");
    Ok(SystemCredentials{ login: shards.next().unwrap().into(), password: shards.next().unwrap().into() })
}

pub fn write_scc_credentials(credentials: &SystemCredentials) -> Result<(), Error> {
  debug!("Writing {:?} into SCCcredentials file", credentials);

  fs::create_dir_all("/etc/zypp/credentials.d").unwrap();
  let mut scc_credentials = try!(fs::File::create("/etc/zypp/credentials.d/SCCcredentials"));
  try!(scc_credentials.write_fmt(format_args!("{}:{}", credentials.login, credentials.password)));

  Ok(())
}
