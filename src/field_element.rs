use std::ops::{Add, Sub, Mul, Div};

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

    // Couldn't find a `**` operator for Rust, so exponentiation is a function
    pub fn exp(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime { panic!("both numbers need to be in same field")}
        let num: i32 = self.num;
        let other = other.num as u32;
        let exp_num = (num.pow(other) % self.prime);
        FieldElement::new(exp_num, self.prime)
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

// we use Fermat's Little Theorem for division
impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime { panic!("both numbers need to be in same field")}
        let prime = self.prime as u32;
        let exp_prime_minus_two = (other.num.pow(prime - 2)) as i32;
        let fermattted_num = ((self.num * exp_prime_minus_two) % self.prime) as i32;
        FieldElement::new(fermattted_num, self.prime)
    }
}



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

    #[test]
    fn field_element_div_test() {
        let a = FieldElement::new(2, 19);
        let b = FieldElement::new(3, 19);
        let c = a / b;
        assert_eq!(c.num, 7);
    }

    #[test]
    fn field_element_div2_test() {
        let a = FieldElement::new(11, 19);
        let b = FieldElement::new(2, 19);
        let c = a / b;
        assert_eq!(c.num, 15);
    }

    #[test]
    fn field_element_exp_test() {
        let a = FieldElement::new(3, 19);
        let b = FieldElement::new(3, 19);
        let c = a.exp(b);
        assert_eq!(c.num, 8);
    }

}