extern crate crates_io;

use std::env;
use crates_io::{Registry};

const HOST: &'static str = "https://crates.io";

///api/v1/crates?q=mail&per_page=10


fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() > 0, "Usage: ./kalfred-crates [query]");

    let ref query = args[1];

    let mut registry = Registry::new(String::from(HOST), None);

    match registry.search(&query, 10) {
        Ok((crates, _)) => {
            /*let list_items = crates.iter()
                .map(|krate| (
                    format!("{} ({})", krate.name, krate.max_version),
                    krate.description.as_ref()
                ))
                .collect::<Vec<_>>();*/
            println!("<?xml version=\"1.0\"?>");
            println!("<items>");
            // https://www.alfredapp.com/help/workflows/inputs/script-filter/xml/
            crates.iter().inspect(|krate| println!("<item arg=\"{}/crates/{}\" type=\"url\"><title>{}</title><subtitle>{}</subtitle></item>", HOST, krate.name,krate.name, krate.description.clone().unwrap_or(String::from("OK")))).collect::<Vec<_>>();
            println!("</items>");
            return ();
        },
        Err(err) => println!("An error occured")
    }

    //let crates = json::decode::<Crates>(&body)?;
    //println!("{:?}", crates);

}
/*
fn req(&mut self,
       path: String,
       body: Option<&[u8]>,
       authorized: Auth) -> Result<String> {
    self.handle.url(&format!("{}/api/v1{}", self.host, path))?;
    let mut headers = List::new();
    headers.append("Accept: application/json")?;
    headers.append("Content-Type: application/json")?;

    if authorized == Auth::Authorized {
        let token = match self.token.as_ref() {
            Some(s) => s,
            None => return Err(Error::TokenMissing),
        };
        headers.append(&format!("Authorization: {}", token))?;
    }
    self.handle.http_headers(headers)?;
    match body {
        Some(mut body) => {
            self.handle.upload(true)?;
            self.handle.in_filesize(body.len() as u64)?;
            handle(&mut self.handle, &mut |buf| body.read(buf).unwrap_or(0))
        }
        None => handle(&mut self.handle, &mut |_| 0),
    }
*/
