extern crate clap;
use clap::{Arg, App};

fn main() {
  let _matches = App::new("uconnect")
                    .version("0.1.0")
                    .author("Kirill Pimenov <kpimenov@suse.de>")
                    .about("Rust reimplementation of the SUSE Connect tool.")
                    .arg(Arg::with_name("debug")
                             .short("d")
                             .long("debug")
                             .help("Enable debugging output"))
                    .get_matches();
  println!("Hello, world!");
}
