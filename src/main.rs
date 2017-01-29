extern crate crates_io;

use std::env;
use crates_io::{Registry};

const HOST: &'static str = "https://crates.io";

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 0, "Usage: ./alfred-crates [query]");
    let ref query = args[1];
    let mut registry = Registry::new(String::from(HOST), None);
    match registry.search(&query, 10) {
        Ok((crates, _)) => {
            println!("<?xml version=\"1.0\"?>");
            println!("<items>");

            // https://www.alfredapp.com/help/workflows/inputs/script-filter/xml/
            crates.iter().inspect(|krate| println!("<item arg=\"{}/crates/{}\" type=\"url\"><title>{}</title><subtitle>{}</subtitle></item>", HOST, krate.name,krate.name, krate.description.clone().unwrap_or(String::from("")))).collect::<Vec<_>>();
            println!("</items>");
            return ();
        },
        Err(err) => println!("An error occurred")
    }
}
