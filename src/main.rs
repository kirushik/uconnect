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

  if matches.is_present("debug") {
    enable_debug();
  };

  // Calling `unwrap()` should be safe, because regcode presence is validated by Clap setup
  let regcode = matches.value_of("REGCODE").unwrap();
  // TODO Properly handle hostnames without `http://` here
  let server_url = matches.value_of("URL").unwrap_or("https://scc.suse.com");

  announce_system(&regcode, &server_url, &http_client).unwrap();
}


// Logging
#[macro_use]
extern crate log;
extern crate flexi_logger;

fn enable_debug() {
  flexi_logger::init(flexi_logger::LogConfig::new(), Some("uconnect=debug".to_string())).unwrap();
}


// HTTP client
extern crate hyper;
use hyper::client::Client;
use hyper::Url;
use hyper::header::{Accept, Authorization, ContentType, AcceptEncoding, Encoding, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use std::io::Read; // To make `read_to_string` work

fn announce_system<'a>(regcode: &str, server_url: &str, http_client: &Client) -> hyper::error::Result<()> {
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
  write_scc_credentials(&response_body);

  Ok(())
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

fn write_scc_credentials(json_response: &str) {
  let credentials: SystemCredentials = json::decode(json_response).unwrap(); // TODO Solve issue with Error inheritance and use try! here
  debug!("{:?}", credentials);
}
