/// the below is for general elliptic curves that satisfy the equation:
/// y2=x3+ax+b

// TODO: refactor Point so that it takes in Field Elements

use std::ops::{Add};
use field_element::{FieldElement};

#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub a: i32,
    pub b: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, a: i32, b: i32) -> Point {
        // 0 is equivalent to `None` for now, need to just return b/c won't
        // satisfy curve
        if x == 0 && y == 0 { return Point { x: x, y: y, a: a, b: b } }
        if y.pow(2) != (x.pow(3) + (a * x + b)) { panic!("Point {}, {} is not on the curve where a,b = {},{}", x, y, a,b) }
        Point { x: x, y: y, a: a, b: b }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        if (self.a != other.a || self.b != other.b) { panic!("Points are not on the same curve!")}
        // handle infinity points
        if self.x == 0 { return Point { x: other.x, y: other.y, a: other.a, b: other.b } }
        if other.x == 0 { return Point { x: self.x, y: self.y, a: self.a, b: self.b } }
        // hanlde result that is at point infinity
        if self.x == other.x && self.y != other.y { return Point { x: 0, y: 0, a: self.a, b: self.b } }
        if self.x != other.x {
            // find the slope: s=(y2-y1)/(x2-x1)
            let s = (other.y - self.y)/(other.x - self.x);
            // find x of third point x3: x3=s2-x1-x2
            let x3 = s.pow(2) - self.x - other.x;
            // find y of third point: y3=s(x1-x3)-y1
            let y3 = s * (self.x - x3) - self.y;

            Point { x: x3, y: y3, a: self.a, b: self.b }
        } else {
            // s=(3*x1**2+a)/(2*y1)
            let s = (3 * (self.x).pow(2) + self.a ) / (2 * self.y);
            // x3=s**2-2*x1
            let x3 = s.pow(2) - (2 * self.x);
            // y3=s*(x1-x3)-y1
            let y3 = s * (self.x - x3) - self.y;

            Point { x: x3, y: y3, a: self.a, b: self.b }
        }
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecc_new_test() {
        let p1 = Point::new(-1, -1, 5, 7);
        let p2 = Point { x: -1, y: -1, a: 5, b: 7 };

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_add0(){
        // this test is currently failing, issues with dealing with `None`
        let p1 = Point::new(0, 0, 5, 7);
        let p2 = Point::new(2, 5, 5, 7);
        let p3 = Point::new(2, 5, 5, 7);
        
        assert_eq!(p1 + p2, p3);
        // assert_eq!(p2 + p1, p3);
        // assert_eq!(p2 + p3, p1);

    }

    #[test]
    fn test_add1(){
        let p1 = Point::new(3, 7, 5, 7);
        let p2 = Point::new(-1, -1, 5, 7);
        let p3 = Point::new(2, -5, 5, 7);
        
        // assert_eq!(p1 + p2, p3);
        // assert_eq!(p2 + p1, p3);
        assert_eq!(p1 + p2, p3);

    }

    #[test]
    fn comb_point_ecc0(){
        let a = FieldElement::new(1, 2);
        // let p1 = Point::new(Some(3), Some(7), 5, 7);
        // let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        // let p3 = Point::new(Some(2), Some(-5), 5, 7);
        
        // assert_eq!(p1 + p2, p3);
        // assert_eq!(p2 + p1, p3);
        assert_eq!(a, FieldElement{ num: 1, prime: 2 });

    }


}