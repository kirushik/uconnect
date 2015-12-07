use std::result::Result;

extern crate xml;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

pub fn base_product() -> Result<String, &'static str> {
    let output = installed_products();
    debug!("call resulted in {:?}", output);
    Ok("SLES".into())
}

fn installed_products() -> Vec<Product> {
    //let xml_output = call("--no-refresh --xmlout --non-interactive products -i");
    //debug!("Zypper returned\n{}", xml_output);
    let xml_output = r##"
        <?xml version='1.0'?>
        <stream>
        <message type="info">Loading repository data...</message>
        <message type="info">Reading installed packages...</message>
        <product-list>
        <product name="openSUSE" version="42.1" release="0" epoch="0" arch="x86_64" vendor="openSUSE" summary="openSUSE" repo="@System" productline="Leap" registerrelease="" shortname="openSUSE" flavor="ftp" isbase="true" installed="true"><endoflife time_t="0" text="0"/><registerflavor/><description>openSUSE Leap</description></product>
        </product-list>
        </stream>
    "##;
    parse_products(xml_output)
}

#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub version: String,
    pub arch: String
}

fn parse_products(xml: &str) -> Vec<Product> {
    let mut products = vec![];
    let parser = EventReader::from_str(xml);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement{name, attributes, namespace}) => {
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

    for attr in attributes {
        match attr.name.local_name.as_ref() {
            "name" => name = attr.value.as_ref(),
            "version" => version = attr.value.as_ref(),
            "arch" => arch = attr.value.as_ref(),
            _ => {}
        }
    }
    Product{name: name.into(), version: version.into(), arch: arch.into()}
}

fn call(arguments: &str) -> String {
    use std::process::Command;
    let arguments : Vec<&str> = arguments.split_whitespace().collect();

    let output = Command::new("zypper").args(&arguments).output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}
