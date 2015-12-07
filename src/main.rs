// Options parsing
extern crate clap;
use clap::{Arg, App, AppSettings};

// Logging
#[macro_use]
extern crate log;
extern crate flexi_logger;

// HTTP client
extern crate hyper;

// JSON support
extern crate rustc_serialize;

// XML support
extern crate xml;

mod logging;
mod connect_api;
mod scc_credentials;
mod zypper;

fn main() {
    let http_client = hyper::client::Client::new();

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

    logging::enable_logging(matches.is_present("debug"));

    // Calling `unwrap()` should be safe, because regcode presence is validated by Clap setup
    let regcode = matches.value_of("REGCODE").unwrap();
    // TODO Properly handle hostnames without `http://` here
    let server_url = matches.value_of("URL").unwrap_or("https://scc.suse.com");

    let scc_credentials = scc_credentials::SystemCredentials::read().unwrap_or_else(|_error| {
        match connect_api::announce_system::announce_system(&regcode, &server_url, &http_client) {
            Ok(credentials) => { credentials.write().unwrap(); credentials },
            Err(x) => {
                error!("{}", x);
                std::process::exit(67);
            }
        }
    });

    // TODO implement handling of `--product` option here
    let product = zypper::base_product().unwrap();
    connect_api::activate_product::activate_product(&product, &scc_credentials, &regcode, &server_url, &http_client).unwrap();
}
