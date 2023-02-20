use clap::{App, Arg};

mod crt;
use crt::get_main;

mod alien_vault;
use alien_vault::alien;

fn main() {
    let matches = App::new("domni")
        .version("0.1.0")
        .author("R4vanan <devilface1999@gmail.com>")
        .about("A command line tool for domain intelligence gathering")
        .arg(Arg::with_name("DOMAIN")
            .help("Sets the domain to search")
            .required(true)
            .index(1))
        .get_matches();

    let domain = matches.value_of("DOMAIN").unwrap();
    get_main(domain).expect("domain not found!");
    alien(domain).expect("domain not found!");
}

