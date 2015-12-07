use std::result::Result;

extern crate serde;
extern crate serde_xml;

use serde_xml::from_str;

use std::collections::HashMap;

pub fn base_product() -> Result<String, &'static str> {
    let output = installed_products();
    debug!("call resulted in {:?}", output);
    Ok("SLES".into())
}

fn installed_products() -> HashMap<String, String> {
    let xml_output = call("--no-refresh --xmlout --non-interactive products -i");
    from_str(&xml_output).unwrap()
}

fn call(argument: &str) -> Vec<u8> {
    use std::process::Command;

    // TODO proper error handling here
    let output = Command::new("echo").arg(argument).output().unwrap();
    output.stdout
}
