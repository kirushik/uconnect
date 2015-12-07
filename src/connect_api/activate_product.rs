use scc_credentials::SystemCredentials;
//use connect_api::errors::Result;
use std::result::Result;

use hyper::client::Client;

use zypper::Product;

pub fn activate_product(product: &Product, credentials: &SystemCredentials, server_url: &str, http_client: &Client) -> Result<(), &'static str> {
  debug!("Calling activate_product for {:?}", product);
  Err("Not yet implemented")
}
