use hyper::client::Client;
use hyper::error::Result;
use hyper::Url;
use hyper::header::{Accept, Authorization, ContentType, AcceptEncoding, Encoding, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};

use rustc_serialize::json;

use std::io::Read; // to make `read_to_string` work

use scc_credentials::SystemCredentials;

use std::process::Command;

pub fn announce_system(regcode: &str, server_url: &str, http_client: &Client) -> Result<SystemCredentials> {
  debug!("Provided regcode {:?}", regcode);
  debug!("Calling SCC server at URL {:?}", server_url);

  let url = try!(Url::parse(&format!("{}/connect/subscriptions/systems", server_url)));
  let payload = AnnouncePayload::read().to_json();

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

#[derive(RustcEncodable, Debug)]
struct HwInfo {
  arch: Option<String>,
  cpus: Option<u32>,
  sockets: Option<u32>,
  hypervisor: Option<String>,
  uuid: Option<String>
}

#[derive(RustcEncodable, Debug)]
struct AnnouncePayload {
  hostname: String,
  hw_info: HwInfo
}

impl AnnouncePayload {
    fn read() -> AnnouncePayload {
        let mut arch: Option<String> = None;
        let mut cpus: Option<u32> = None;
        let mut sockets: Option<u32> = None;
        let mut hypervisor: Option<String> = None;

        let lscpu_data = String::from_utf8(Command::new("lscpu").output().unwrap().stdout).unwrap();
        let lines = lscpu_data.split("\n");
        for line in lines {
            let mut chunks = line.trim().split_whitespace();
            match chunks.next() {
                Some("Architecture:") => arch = Some(chunks.next().unwrap().into()),
                Some("CPU(s):") => cpus = Some(chunks.next().unwrap().parse().unwrap()),
                Some("Socket(s):") => sockets = Some(chunks.next().unwrap().parse().unwrap()),
                Some("Hypervisor") => { // Actual title is "Hypervisor vendor:"
                    chunks.next().unwrap(); // Ignoring "vendor:" part
                    hypervisor = Some(chunks.next().unwrap().into());
                }
                _ => {}
            }
        }

        let result = AnnouncePayload {
            hostname: "ignis".into(),
            hw_info: HwInfo {
                arch: arch,
                cpus: cpus,
                sockets: sockets,
                hypervisor: hypervisor,
                uuid: Some("67a13430-48c5-4454-b9b9-46010ac0e391".into())
            }
        };

        debug!("Detected HwInfo: {:?}", result);
        result
    }

    fn to_json(&self) -> String {
        json::encode(&self).unwrap()
    }
}
