use std::result::Result;

extern crate xml;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

use scc_credentials::SystemCredentials;

pub fn base_product() -> Result<Product, &'static str> {
    let products = installed_products();
    debug!("Zypper listed the following products: {:?}", products);
    for product in products {
        if product.is_base {
            return Ok(product)
        }
    }
    Err("base product not found")
}

pub fn installed_products() -> Vec<Product> {
    let xml_output = call("--no-refresh --xmlout --non-interactive products -i");
    parse_products(&xml_output)
}

pub fn add_service(service: &Service) -> Result<(), &'static str> {
    try!(remove_service(service));
    call(&format!("--non-interactive addservice -t ris {} {}", service.url, service.name)); // adding the service record
    call(&format!("--non-interactive modifyservice -r {}", service.name)); // enabling service autorefresh

    SystemCredentials::read().unwrap().write_for_service(&service.name).unwrap();

    Ok(())
}

pub fn remove_service(service: &Service) -> Result<(), &'static str> {
    call(&format!("--non-interactive removeservice {}", service.name));

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Product {
    pub identifier: String,
    pub version: String,
    pub arch: String,
    pub is_base: bool,
    pub installed: bool
}

#[derive(RustcDecodable, Debug)]
pub struct Service {
    pub name: String,
    pub url: String
}

fn parse_products(xml: &str) -> Vec<Product> {
    let mut products = vec![];
    let parser = EventReader::from_str(xml);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement{name, attributes, ..}) => {
                if name.local_name == "product" {
                    products.push(extract_product(&attributes))
                }
            }
            _ => {}
        }
    }
    products
}

fn extract_product(attributes: &[OwnedAttribute]) -> Product {
    let mut identifier = "unknown";
    let mut version = "unknown";
    let mut arch = "unknown";
    let mut is_base = false;
    let mut installed = false;

    for attr in attributes {
        match attr.name.local_name.as_ref() {
            "name" => identifier = attr.value.as_ref(),
            "version" => version = attr.value.as_ref(),
            "arch" => arch = attr.value.as_ref(),
            "isbase" => is_base = attr.value=="true",
            "installed" => installed = attr.value=="true",
            _ => {}
        }
    }
    Product{identifier: identifier.into(), version: version.into(), arch: arch.into(), is_base: is_base, installed: installed}
}

fn call(arguments: &str) -> String {
    use std::process::Command;
    let arguments : Vec<&str> = arguments.split_whitespace().collect();

    debug!("Calling zypper with arguments {:?}", arguments);
    let output = Command::new("zypper").args(&arguments).output().unwrap();
    debug!("Zypper exited with status {:?}", output.status);

    String::from_utf8(output.stdout).unwrap()
}
