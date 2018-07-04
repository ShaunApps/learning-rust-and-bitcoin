extern crate sha2;
extern crate hyper;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::{Value, Error};

use sha2::{Sha256, Digest};
use std::env;
use hyper::rt::{self, Future, Stream};

pub mod utxos;
pub mod field_element;
pub mod ecc;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;


/// ConfirmationCache as bool.
#[derive(Deserialize, Debug)]
struct ConfirmationCache(bool);
impl Default for ConfirmationCache {
    fn default() -> Self {
        ConfirmationCache(false)
    }
}

#[derive(Deserialize, Debug)]
struct UTXO {
    address: String,
    txid: String,
    vout: u8,
    ts: i32,
    scriptPubKey: String,
    amount: f64,
    confirmations: u8,

    #[serde(default)]
    confirmationsFromCache: ConfirmationCache,
}

// takes json string data, deserializes it,
// sorts by ts and returns it
fn parse_and_sort_utxos_by_date(utxos: &str) -> Vec<UTXO> {
    let mut json: Vec<UTXO> =
        serde_json::from_str(utxos).expect("JSON was not well-formatted");
    json.sort_by(|a, b| a.ts.cmp(&b.ts));
    json
}

// fn find_range(utxos: Vec<UTXO>, x: f64) -> (i32, i32) {

// }


fn main() {
   // uses hyper to GET blockchain.info API with address and return body
//    rt::run(utxos::fetch_utxos(utxos::get_address().unwrap()));

// let mut data = File::open("utxo.json").unwrap();
// let mut contents = String::new();
//     data.read_to_string(&mut contents)
//         .expect("something went wrong reading the file");

//     println!("With text:\n{}", contents);

    


let data = r#"[
                {
                    "address": "n2PuaAguxZqLddRbTnAoAuwKYgN2w2hZk7",
                    "txid": "dbfdc2a0d22a8282c4e7be0452d595695f3a39173bed4f48e590877382b112fc",
                    "vout": 0,
                    "ts": 1401276201,
                    "scriptPubKey": "76a914e50575162795cd77366fb80d728e3216bd52deac88ac",
                    "amount": 0.001,
                    "confirmations": 3
                    },
                    {
                    "address": "n2PuaAguxZqLddRbTnAoAuwKYgN2w2hZk7",
                    "txid": "e2b82af55d64f12fd0dd075d0922ee7d6a300f58fe60a23cbb5831b31d1d58b4",
                    "vout": 0,
                    "ts": 1401226410,
                    "scriptPubKey": "76a914e50575162795cd77366fb80d728e3216bd52deac88ac",
                    "amount": 0.05,
                    "confirmations": 6,
                    "confirmationsFromCache": true
                },
                {
                    "address": "n2PuaAguxZqLddRbTnAoAuwKYgN2w2hZk7",
                    "txid": "e2b82af55d64f12fd0dd075d0922ee7d6a300f58fe60a23cbb5831b31d1d58b4",
                    "vout": 1,
                    "ts": 1301226440,
                    "scriptPubKey": "76a914e50575162795cd77366fb80d728e3216bd52deac88ac",
                    "amount": 0.00134,
                    "confirmations": 6,
                    "confirmationsFromCache": true
                },
                {
                    "address": "n2PuaAguxZqLddRbTnAoAuwKYgN2w2hZk7",
                    "txid": "e2b82af55d64f12fd0dd075d0922ee7d6a300f58fe60a23cbb5831b31d1d58b4",
                    "vout": 2,
                    "ts": 1351226440,
                    "scriptPubKey": "76a914e50575162795cd77366fb80d728e3216bd52deac88ac",
                    "amount": 0.0031,
                    "confirmations": 6,
                    "confirmationsFromCache": true
                }

            ]"#;

    // let mut json: Vec<UTXO> =
    //     serde_json::from_str(data).expect("JSON was not well-formatted");

    
    // json.sort_by(|a, b| a.ts.cmp(&b.ts));
    // for i in &mut json {
    //     println!("test utxo ts: {:?}\n", i.ts);
    // }


    let mut sorted_json = parse_and_sort_utxos_by_date(data);

    for i in &mut sorted_json {
        println!("test utxo ts: {:?}\n", i.ts);
    }


    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_hello_world_test() {
        let mut hasher = Sha256::default();
        hasher.input("hello world".as_bytes());
        let hash = hasher.result();
    
        assert_eq!(hash[..], [0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08,
                                0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d, 0xab, 0xfa,
                                0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee,
                                0x90, 0x88, 0xf7, 0xac, 0xe2, 0xef, 0xcd, 0xe9]);
    }

}