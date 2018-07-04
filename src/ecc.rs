
#[derive(PartialEq, Eq, Debug)]
pub struct Point {
    pub a: i32,
    pub b: i32,
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, a: i32, b: i32) -> Point {
        if y.pow(2) != (x.pow(3) + (a * x) + b) { panic!("Point {}, {} is not on the curve where a,b = {},{}", x,y,a,b) }
        Point { x: x, y: y, a: a, b: b }
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
    fn ecc_fail_test() {
        let p1 = Point::new(-1, -2, 5, 7);

        assert_eq!(p1.x, -1);
    }

}