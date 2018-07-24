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
pub mod elliptic_curves;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;




fn main() {    


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