use std::ops::{Add};

#[derive(PartialEq, Eq, Debug)]
pub struct Point {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub a: i32,
    pub b: i32,
}

impl Point {
    pub fn new(x: Option<i32>, y: Option<i32>, a: i32, b: i32) -> Point {
        // maybe add case where x,y is point at infinity (None for Rust?)
        if x == None && y == None { () }
        if y.unwrap().pow(2) != (x.unwrap().pow(3) + (a * x.unwrap()) + b) { panic!("Point {}, {} is not on the curve where a,b = {},{}", x.unwrap(),y.unwrap(),a,b) }
        Point { x: x, y: y, a: a, b: b }
    }
}

// TODO: implement Point addition
impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        if (self.a != other.a || self.b != other.b) { panic!("Points are not on the same curve!")}
        // handle infinity points
        if self.x == None { return Point { x: other.x, y: other.y, a: other.a, b: other.b } }
        if other.x == None { return Point { x: self.x, y: self.y, a: self.a, b: self.b } }
        // hanlde result that is at point infinity
        if self.x == other.x && self.y != other.y { return Point { x: None, y: None, a: self.a, b: self.b } }
        if self.x != other.x {
            // find the slope: s=(y2-y1)/(x2-x1)
            let s = (other.y.unwrap() - self.y.unwrap())/(other.x.unwrap() - self.x.unwrap());
            // find x of third point x3: x3=s2-x1-x2
            let x3 = s.pow(2) - self.x.unwrap() - other.x.unwrap();
            // find y of third point: y3=s(x1-x3)-y1
            let y3 = s * (self.x.unwrap() - x3) - self.y.unwrap();

            Point { x: Some(x3), y: Some(y3), a: self.a, b: self.b }
        } else {
            // s=(3*x1**2+a)/(2*y1)
            let s = (3 * (self.x.unwrap()).pow(2) + self.a ) / (2 * self.y.unwrap());
            // x3=s**2-2*x1
            let x3 = s.pow(2) - (2 * self.x.unwrap());
            // y3=s*(x1-x3)-y1
            let y3 = s * (self.x.unwrap() - x3) - self.y.unwrap();

            Point { x: Some(x3), y: Some(y3), a: self.a, b: self.b }
        }
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecc_new_test() {
        let p1 = Point::new(Some(-1), Some(-1), 5, 7);
        let p2 = Point { x: Some(-1), y: Some(-1), a: 5, b: 7 };

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_add0(){
        let p1 = Point::new(None, None, 5, 7);
        let p2 = Point::new(Some(2), Some(5), 5, 7);
        let p3 = Point::new(Some(2), Some(5), 5, 7);
        
        assert_eq!(p1 + p2, p3);
        // assert_eq!(p2 + p1, p3);
        // assert_eq!(p2 + p3, p1);

    }

    #[test]
    fn test_add1(){
        let p1 = Point::new(Some(3), Some(7), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(2), Some(-5), 5, 7);
        
        // assert_eq!(p1 + p2, p3);
        // assert_eq!(p2 + p1, p3);
        assert_eq!(p1 + p2, p3);

    }


}