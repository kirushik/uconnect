use std::result::Result;

extern crate xml;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

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

fn installed_products() -> Vec<Product> {
    let xml_output = call("--no-refresh --xmlout --non-interactive products -i");
    parse_products(&xml_output)
}

#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub is_base: bool,
    pub installed: bool
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
    let mut name = "unknown";
    let mut version = "unknown";
    let mut arch = "unknown";
    let mut is_base = false;
    let mut installed = false;

    for attr in attributes {
        match attr.name.local_name.as_ref() {
            "name" => name = attr.value.as_ref(),
            "version" => version = attr.value.as_ref(),
            "arch" => arch = attr.value.as_ref(),
            "isbase" => is_base = (attr.value == "true"),
            "installed" => installed = (attr.value == "true"),
            _ => {}
        }
    }
    Product{name: name.into(), version: version.into(), arch: arch.into(), is_base: is_base, installed: installed}
}

fn call(arguments: &str) -> String {
    use std::process::Command;
    let arguments : Vec<&str> = arguments.split_whitespace().collect();

    let output = Command::new("zypper").args(&arguments).output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}
