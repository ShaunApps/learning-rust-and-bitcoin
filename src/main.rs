extern crate sha2;
use sha2::{Sha256, Digest};
pub mod field_element;
use std::env;


fn get_address_and_fetch() -> Result<String, &'static str> {

    let mut url: String = "http://blockchain.info/rawaddr/".to_owned();

    let address = match env::args().nth(2) {
        Some(address) => address,
        None => {
            return Err("Usage: address <address>");
        }
    };
    url.push_str(&address);
    println!("{}", url);
    Ok(url)
}



fn main() {
    get_address_and_fetch();
    
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