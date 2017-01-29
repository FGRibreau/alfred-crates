extern crate crates_io;

use std::env;
use crates_io::{Registry, Crate};

const HOST: &'static str = "https://crates.io";

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 0, "Usage: ./alfred-crates [query]");

    let ref query = args[1];
    let mut registry = Registry::new(String::from(HOST), None);

    match registry.search(&query, 10) {
        Ok((crates, _)) => {
            workflow_output(crates);
            return ();
        },
        Err(_) => {
            // @todo find a way in alfred to inform about the error
            workflow_output(vec![])
        }
    }
}

fn workflow_output(crates: Vec<Crate>){
    println!("<?xml version=\"1.0\"?>");
    println!("<items>");
    // https://www.alfredapp.com/help/workflows/inputs/script-filter/xml/
    crates.iter().inspect(|krate| println!("<item arg=\"{}/crates/{}\" type=\"url\"><title><![CDATA[{}]]></title><subtitle><![CDATA[{}]]></subtitle></item>", HOST, krate.name,krate.name, krate.description.clone().unwrap_or(String::from("")))).collect::<Vec<_>>();
    println!("</items>");
}
