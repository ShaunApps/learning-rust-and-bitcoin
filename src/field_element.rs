use std::ops::{Add, Sub, Mul};

#[derive(PartialEq, Eq, Debug)]
pub struct FieldElement { 
    pub num: i32,
    pub prime: i32,
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> FieldElement {
        if num < 0 || num >= prime { panic!("num should be between 0 and {}-1", prime) }
        FieldElement { num: num, prime: prime }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime { panic!("both numbers need to be in same field")}
        let result = (self.num + other.num) % self.prime;
        FieldElement { num: result, prime: self.prime }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime { panic!("both numbers need to be in same field")}
        let mut result = (self.num - other.num) % self.prime;
        if result < 0 {
            result = result + self.prime;
        }
        FieldElement { num: result, prime: self.prime }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime { panic!("both numbers need to be in same field")}
        let result = (self.num * other.num) % self.prime;
        FieldElement { num: result, prime: self.prime }
    }
}

// TODO: implement Exp and Div for FieldElement





#[cfg(test)]
mod tests {
    use super::*;


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

    #[test]
    fn field_element_mul_test() {
        let a = FieldElement::new(9, 13);
        let b = FieldElement::new(3, 13);
        let c = a * b;
        assert_eq!(c.num, 1);
    }

}