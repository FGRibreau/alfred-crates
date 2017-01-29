extern crate crates_io;
extern crate xml;
extern crate clap;

use clap::{Arg, App};
use xml::escape::escape_str_pcdata;
use crates_io::{Registry, Crate};

const HOST: &'static str = "https://crates.io";

fn main() {
    // access metadata from cargo package http://stackoverflow.com/a/27841363/745121
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("query")
            .short("q")
            .required(true)
            .index(1))
        .get_matches();

    let query = args.value_of("query").unwrap();
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
    for krate in crates{
        let escaped_name = escape_str_pcdata(&krate.name);
        println!("<item arg=\"{}/crates/{}\" type=\"url\"><title><![CDATA[{}]]></title><subtitle><![CDATA[{}]]></subtitle></item>", HOST, escaped_name,escaped_name, escape_str_pcdata(&krate.description.clone().unwrap_or(String::from(""))));
    }
    println!("</items>");
}
