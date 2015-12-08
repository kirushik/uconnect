use std::fs::create_dir_all;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::{Write, BufRead, BufReader};

#[derive(RustcDecodable, Debug)]
pub struct SystemCredentials {
  pub login: String,
  pub password: String
}

impl SystemCredentials {
    pub fn read() -> Result<SystemCredentials, Error> {
        let file = try!(File::open("/etc/zypp/credentials.d/SCCcredentials"));
        let mut reader = BufReader::new(&file);

        //TODO Two buffers are a hack, and I can do better
        let mut buffer1 = String::new();
        let login: Option<&str> = {
          try!(reader.read_line(&mut buffer1));
          let mut shards = buffer1.trim().split("=");
          shards.nth(1)
        };

        let mut buffer2 = String::new();
        let password: Option<&str> = {
          try!(reader.read_line(&mut buffer2));
          let mut shards = buffer2.trim().split("=");
          shards.nth(1)
        };

        if let Some(login) = login {
          if let Some(password) = password {
            debug!("Loaded existing SCC credentials {:?}", (login, password));
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
        try!(self.write_into("SCCcredentials"));
        Ok(())
    }

    pub fn write_for_service(&self, service_name: &str) -> Result<(), Error> {
        debug!("Writing {:?} into {} credentials file", self, service_name);
        try!(self.write_into(service_name));
        Ok(())
    }

    fn write_into(&self, filename: &str) -> Result<(), Error> {
        try!(create_dir_all("/etc/zypp/credentials.d"));
        let mut scc_credentials = try!(File::create(format!("/etc/zypp/credentials.d/{}", filename)));
        try!(scc_credentials.write_fmt(format_args!("username={}\npassword={}", self.login, self.password)));

        Ok(())
    }
}
