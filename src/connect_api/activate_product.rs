use scc_credentials::SystemCredentials;
use connect_api::errors::Result;

use hyper::client::Client;
use hyper::Url;
use hyper::header::{Accept, Authorization, Basic, ContentType, AcceptEncoding, Encoding, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};

use zypper::Product;

pub fn activate_product(product: &Product, credentials: &SystemCredentials, regcode: &str, server_url: &str, http_client: &Client) -> Result<()> {
  debug!("Calling activate_product for {:?}", product);

  let url = Url::parse(&format!("{}/connect/systems/products", server_url)).unwrap();
  let payload = ActivateProductPayload::prepare(product, regcode).to_json();

  let request = http_client.post(url)
                           .header(Authorization(Basic{
                             username: credentials.login.to_owned(),
                             password: Some(credentials.password.to_owned())
                           }))
                           .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.scc.suse.com.v4+json".into()), vec![]))]))
                           .header(ContentType::json())
                           .header(AcceptEncoding(vec![qitem(Encoding::Gzip), qitem(Encoding::Deflate)]))
                           .body(&payload);
  let mut response = try!(request.send());
  debug!("HTTP response status is {:?}", response.status);

  Ok(())
}

#[derive(RustcEncodable, Debug)]
struct ActivateProductPayload;

impl ActivateProductPayload {
  fn prepare(product: &Product, regcode: &str) -> ActivateProductPayload {
    ActivateProductPayload
  }

  fn to_json(&self) -> String {
    use rustc_serialize::json;
    json::encode(&self).unwrap()
  }
}
