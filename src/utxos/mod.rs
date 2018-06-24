extern crate hyper;
extern crate serde_json;

use std::env;
use std::io::{self, Write};
use std::str;

use hyper::Client;
use hyper::rt::{self, Future, Stream};

use serde_json::{Value, Error};


// #[derive(Serialize, Deserialize)]
// struct Block {
//     none: usize,
// }

// pub fn fetch_utxos(url: hyper::Uri) -> impl Future<Item=(), Error=()> {
//     let client = Client::new();

//     client
//         // Fetch the url...
//         .get(url)
//         // And then, if we get a response back...
//         .and_then(|res| {
//             // The body is a stream, and for_each returns a new Future
//             // when the stream is finished, and calls the closure on
//             // each chunk of the body...
//             res.into_body().for_each(|chunk| {
//                 let as_str = str::from_utf8(&chunk).unwrap();
//                 let v: Value = serde_json::from_str(as_str);

//                 // // Access parts of the data by indexing with square brackets.
//                 // println!("nonce value {}", v["nonce"]);

//                 io::stdout().write_all(v["nonce"])
//                     .map_err(|e| panic!("example expects stdout is open, error={}", e))
//             })
//         })
//         // If all good, just tell the user...
//         .map(|_| {
//             println!("\n\nDone.");
//         })
//         // If there was an error, let the user know...
//         .map_err(|err| {
//             eprintln!("Error {}", err);
//         })
// }


// pub fn get_address() -> Result<hyper::Uri, &'static str> {

//     // let mut url: String = "http://blockchain.info/unspent?active=".to_owned();
//     let mut url: String = "http://blockchain.info/rawblock/".to_owned();

//     let address = match env::args().nth(2) {
//         Some(address) => address,
//         None => {
//             return Err("Usage: address <address>");
//         }
//     };

//     url.push_str(&address);
//     println!("full url: {}", url);
//     let url = url.parse::<hyper::Uri>().unwrap();
//     Ok(url)
// }