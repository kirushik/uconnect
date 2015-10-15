use hyper::client::Client;
use hyper::error::Result;
use hyper::Url;
use hyper::header::{Accept, Authorization, ContentType, AcceptEncoding, Encoding, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};

use rustc_serialize::json;

use std::io::Read; // to make `read_to_string` work

use scc_credentials::SystemCredentials;

pub fn announce_system(regcode: &str, server_url: &str, http_client: &Client) -> Result<SystemCredentials> {
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
  Ok(credentials.into())
}

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
