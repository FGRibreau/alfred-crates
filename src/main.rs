extern crate alfred;
extern crate clap;
extern crate crates_io;
extern crate curl;

use clap::{App, Arg};
use crates_io::{Crate, Registry};
use curl::easy::Easy;
use std::io;

const HOST: &'static str = "https://crates.io";

fn main() {
    // access metadata from cargo package http://stackoverflow.com/a/27841363/745121
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("query").short("q").required(true).index(1))
        .get_matches();

    let mut easy_client = Easy::new();
    easy_client
        .useragent("Alfred Crates (robjtede fork)")
        .unwrap();

    let query = args.value_of("query").unwrap();
    let mut registry = Registry::new_handle(String::from(HOST), None, easy_client);

    let mut json = false;
    match registry.search(&query, 10) {
        Ok((crates, _)) => {
            if let Some(version) = alfred::env::version() {
                if version.starts_with("1") || version.starts_with("2") {
                    // only XML support
                    json = false;
                } else {
                    // JSON support
                    json = true;
                }
            }
            workflow_output(crates, json);
            return ();
        }
        Err(_) => {
            // @todo find a way in alfred to inform about the error
            workflow_output(vec![], json)
        }
    }
}

fn workflow_output(crates: Vec<Crate>, json: bool) {
    let items = crates
        .into_iter()
        .map(|item| {
            let url = format!("{}/crates/{}", HOST, item.name);
            alfred::ItemBuilder::new(item.name)
                .arg(url)
                .text_large_type(item.description.clone().unwrap_or(String::from("")))
                .subtitle(item.description.unwrap_or(String::from("")))
                .into_item()
        })
        .collect::<Vec<alfred::Item>>();
    if json {
        alfred::json::Builder::with_items(&items)
            .write(io::stdout())
            .expect("Couldn't write items to Alfred");
    } else {
        alfred::xml::write_items(io::stdout(), &items).expect("Couldn't write items to Alfred");
    }
}
