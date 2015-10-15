// Options parsing
extern crate clap;
use clap::{Arg, App, AppSettings};

fn main() {
    let http_client = Client::new();

    let matches = App::new("uconnect")
                      .version("0.1.0")
                      .author("Kirill Pimenov <kpimenov@suse.de>")
                      .about("Rust reimplementation of the SUSE Connect tool.")

                      .arg(Arg::with_name("debug")
                               .short("d")
                               .long("debug")
                               .global(true)
                               .help("Enable debugging output"))

                      .arg(Arg::with_name("URL")
                               .long("url")
                               .takes_value(true)
                               .help("URL of registration server (e.g. https://scc.suse.com)."))

                      .arg(Arg::with_name("REGCODE")
                               .short("r")
                               .long("regcode")
                               .takes_value(true)
                               .help("Subscription registration code for the product to be registered."))

                      .setting(AppSettings::ArgRequiredElseHelp)

                      .get_matches();

    enable_logging(matches.is_present("debug"));

    // Calling `unwrap()` should be safe, because regcode presence is validated by Clap setup
    let regcode = matches.value_of("REGCODE").unwrap();
    // TODO Properly handle hostnames without `http://` here
    let server_url = matches.value_of("URL").unwrap_or("https://scc.suse.com");

    let scc_credentials = read_scc_credentials().unwrap_or_else(|_error| {
        match announce_system(&regcode, &server_url, &http_client) {
            Ok(credentials) => credentials,
            Err(x) => {
                error!("{}", x);
                std::process::exit(67);
            }
        }
    });
}


// Logging
#[macro_use]
extern crate log;
extern crate flexi_logger;

fn enable_logging(enable_debug: bool) {
  let log_level = if enable_debug {
    Some("uconnect=debug".into())
  } else {
    Some("uconnect=warn".into())
  };
  flexi_logger::init(flexi_logger::LogConfig::new(), log_level).unwrap();
}


// HTTP client
extern crate hyper;
use hyper::client::Client;
use hyper::Url;
use hyper::header::{Accept, Authorization, ContentType, AcceptEncoding, Encoding, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use std::io::prelude::*; // To make `read_to_string` work

fn announce_system(regcode: &str, server_url: &str, http_client: &Client) -> hyper::error::Result<SystemCredentials> {
  debug!("Provided regcode {:?}", regcode);
  debug!("Calling SCC server at URL {:?}", server_url);

  let url = try!(Url::parse(&format!("{}/connect/subscriptions/systems", server_url)));
  let payload = announce_system_payload();

  let request = http_client.post(url)
                           .header(Authorization(format!("Token token=\"{}\"", regcode)))
                           .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.scc.suse.com.v4+json".into()), vec![]))]))
                           .header(ContentType::json())
                           .header(AcceptEncoding(vec![qitem(Encoding::Gzip), qitem(Encoding::Deflate)]))
                           .body(&payload);
  let mut response = try!(request.send());
  debug!("HTTP response status is {:?}", response.status);

  let mut response_body = String::new();
  try!(response.read_to_string(&mut response_body));

  // TODO replace unwrap() with try!() here
  let credentials: SystemCredentials = json::decode(&response_body).unwrap();
  write_scc_credentials(&credentials).unwrap();

  Ok(credentials.into())
}

// JSON support
extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct HwInfo {
  arch: String,
  cpus: u32,
  sockets: u32,
  hypervisor: Option<String>,
  uuid: Option<String>
}

#[derive(RustcEncodable)]
struct AnnouncePayload {
  hostname: String,
  hw_info: HwInfo
}

fn announce_system_payload() -> String {
  let hw_info = AnnouncePayload {
    hostname: "ignis".into(),
    hw_info: HwInfo {
      arch: "x86_64".into(),
      cpus: 2,
      sockets: 2,
      hypervisor: None,
      uuid: Some("67a13430-48c5-4454-b9b9-46010ac0e391".into())
    }
  };
  // TODO Add a proper try! and result throwing here
  json::encode(&hw_info).unwrap()
}

#[derive(RustcDecodable, Debug)]
struct SystemCredentials {
  login: String,
  password: String
}

use std::fs;

fn read_scc_credentials() -> Result<SystemCredentials, std::io::Error> {
    let mut file = try!(fs::File::open("/etc/zypp/credentials.d/SCCcredentials"));

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    debug!("Loaded existing SCC credentials {:?}", buffer);

    let mut shards = buffer.trim().split(":");
    Ok(SystemCredentials{ login: shards.next().unwrap().into(), password: shards.next().unwrap().into() })
}

fn write_scc_credentials(credentials: &SystemCredentials) -> Result<(), std::io::Error> {
  debug!("Writing {:?} into SCCcredentials file", credentials);

  fs::create_dir_all("/etc/zypp/credentials.d").unwrap();
  let mut scc_credentials = try!(fs::File::create("/etc/zypp/credentials.d/SCCcredentials"));
  try!(scc_credentials.write_fmt(format_args!("{}:{}", credentials.login, credentials.password)));

  Ok(())
}
