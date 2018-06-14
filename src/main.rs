extern crate sha2;
use sha2::{Sha256, Digest};
use std::ops::{Add, Sub, Mul};

#[derive(PartialEq, Eq, Debug)]
struct FieldElement { 
    num: i32,
    prime: i32,
}

impl FieldElement {
    fn new(num: i32, prime: i32) -> FieldElement {
        if num < 0 || num >= prime { panic!("num should be between 0 and {}-1", prime) }
        FieldElement { num: num, prime: prime }
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime { panic!("both numbers need to be in same field")}
        let result = (self.num + other.num) % self.prime;
        FieldElement { num: result, prime: self.prime }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime { panic!("both numbers need to be in same field")}
        let mut result = (self.num - other.num) % self.prime;
        if result < 0 {
            result = result + self.prime;
        }
        FieldElement { num: result, prime: self.prime }
    }
}

// impl Mul for FieldElement {
//     type Output = FieldElement;

//     fn mul(self, other: FieldElement) -> FieldElement {
//         if self.prime != other.prime { panic!("both numbers need to be in same field")}
//         let result = (self.num * other.num) % self.prime
//         FieldElement { num: result, prime: self.prime }
//     }
// }


// fn sha256_string(s) -> {
//     let mut hasher = Sha256::default();
//     // write input message
//     hasher.input(s);

//     // read hash digest and consume hasher
//     hasher.result();
// }
    
fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn sha256_hello_world_test() {
    //     hasher.input_str("hello world");

    //     sha256_string()
    
    //     assert_eq!(output[..], [0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08,
    //                             0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d, 0xab, 0xfa,
    //                             0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee,
    //                             0x90, 0x88, 0xf7, 0xac, 0xe2, 0xef, 0xcd, 0xe9]);

    // }

    #[test]
    fn field_element_new_test() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement { num: 7, prime: 13 };
        assert_eq!(a, b);
    }

    #[test]
    fn field_element_add_test() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(8, 13);
        let c = a + b;
        assert_eq!(c.num, 2);
    }

    #[test]
    fn field_element_sub_test() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(8, 13);
        let c = a - b;
        assert_eq!(c.num, 12);
    }

    #[test]
    fn field_element_sub2_test() {
        let a = FieldElement::new(9, 13);
        let b = FieldElement::new(3, 13);
        let c = a - b;
        assert_eq!(c.num, 6);
    }






}
