use std::fs::create_dir_all;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::{Read, Write};

#[derive(RustcDecodable, Debug)]
pub struct SystemCredentials {
  pub login: String,
  pub password: String
}

impl SystemCredentials {
    pub fn read() -> Result<SystemCredentials, Error> {
        let mut file = try!(File::open("/etc/zypp/credentials.d/SCCcredentials"));

        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));

        debug!("Loaded existing SCC credentials {:?}", buffer);

        let mut shards = buffer.trim().split(":");

        if let Some(login) = shards.next() {
          if let Some(password) = shards.next() {
            Ok(SystemCredentials{login: login.into(), password: password.into()})
          } else {
            Err(Error::new(ErrorKind::InvalidData, "SCCcredentials parsing failed: password not found"))
          }
        } else {
          Err(Error::new(ErrorKind::InvalidData, "SCCcredentials parsing failed: login not found"))
        }
    }

    pub fn write(&self) -> Result<(), Error> {
        debug!("Writing {:?} into SCCcredentials file", self);

        try!(create_dir_all("/etc/zypp/credentials.d"));
        let mut scc_credentials = try!(File::create("/etc/zypp/credentials.d/SCCcredentials"));
        try!(scc_credentials.write_fmt(format_args!("{}:{}", self.login, self.password)));

        Ok(())
    }

    pub fn write_for_service(&self, service_name: &str) -> Result<(), Error> {
        debug!("Writing {:?} into {} credentials file", self, service_name);

        try!(create_dir_all("/etc/zypp/credentials.d"));
        let mut scc_credentials = try!(File::create(format!("/etc/zypp/credentials.d/{}", service_name)));
        try!(scc_credentials.write_fmt(format_args!("username={}\npassword={}", self.login, self.password)));

        Ok(())
    }
}
