// Options parsing
extern crate clap;
use clap::{Arg, App, AppSettings};

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
                             .global(true)
                             .help("Enable debugging output"))

                    .arg(Arg::with_name("REGCODE")
                             .short("r")
                             .long("regcode")
                             .takes_value(true)
                             .help("Subscription registration code for the product to be registered."))

                    .setting(AppSettings::ArgRequiredElseHelp)

                    .get_matches();

  if matches.is_present("debug") {
    enable_debug();
  };

  // Calling `unwrap()` should be safe, because regcode presence is validated by Clap setup
  let regcode = matches.value_of("REGCODE").unwrap();
  debug!("Provided regcode {:?}", regcode);

  debug!("It works!");
  println!("Hello, world!");
}

fn enable_debug() {
  flexi_logger::init(flexi_logger::LogConfig::new(), Some("uconnect=debug".to_string())).unwrap();
}
