// Options parsing
extern crate clap;
use clap::{Arg, App};

// Logging
#[macro_use]
extern crate log;
extern crate flexi_logger;

fn main() {
  let matches = App::new("uconnect")
                    .version("0.1.0")
                    .author("Kirill Pimenov <kpimenov@suse.de>")
                    .about("Rust reimplementation of the SUSE Connect tool.")
                    .arg(Arg::with_name("debug")
                             .short("d")
                             .long("debug")
                             .help("Enable debugging output"))
                    .get_matches();

  if matches.is_present("debug") {
    enable_debug();
  };

  debug!("It works!");
  println!("Hello, world!");
}

fn enable_debug() {
  flexi_logger::init(flexi_logger::LogConfig::new(), Some("uconnect=debug".to_string())).unwrap();
}
