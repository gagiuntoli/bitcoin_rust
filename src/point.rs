use crate::finite_field::FiniteField;
use std::ops::{Add, Deref, Div, Mul, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Point {
    Coor {
        a: FiniteField,
        b: FiniteField,
        x: FiniteField,
        y: FiniteField,
    },
    Zero,
}

impl Point {
    fn new(a: FiniteField, b: FiniteField, x: FiniteField, y: FiniteField) -> Point {
        let point = Point::Coor { a, b, x, y };
        if !Self::is_on_curve(&point) {
            panic!("({:?},{:?}) point is not in the curve", x, y);
        }
        point
    }

    fn zero() -> Self {
        Point::Zero
    }

    fn is_zero(self) -> bool {
        self == Point::Zero
    }

    fn is_on_curve(p: &Point) -> bool {
        match p {
            Point::Coor { a, b, x, y } => return y.pow(2) == x.pow(3) + (*a) * (*x) + *b,
            Point::Zero => true,
        }
    }

    fn scale(self, _scalar: u32) -> Self {
        let mut current = self;
        let mut result = Point::Zero;
        let mut scalar = _scalar;

        while scalar != 0 {
            if scalar & 1 != 0 {
                result = result + current;
            }
            current = current + current;
            scalar = scalar >> 1;
        }
        return result;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        match (self, rhs) {
            (Point::Zero, _) => return rhs,
            (_, Point::Zero) => return self,
            (
                Point::Coor { a, b, x, y, .. },
                Point::Coor {
                    a: a_rhs,
                    b: b_rhs,
                    x: x_rhs,
                    y: y_rhs,
                    ..
                },
            ) => {
                if a != a_rhs || b != b_rhs {
                    panic!(
                        "The points (x:{:?},y:{:?},a:{:?},b:{:?}) and (x:{:?},y:{:?},a:{:?},b:{:?}) belong to different curves",
                        x, y, a, b, x_rhs, y_rhs, a_rhs, b_rhs
                    );
                }
                if x != x_rhs {
                    let s = (y_rhs - y) / (x_rhs - x);
                    let x_res = s.pow(2) - x - x_rhs;
                    let y_res = s * (x - x_res) - y;
                    return Point::Coor {
                        a,
                        b,
                        x: x_res,
                        y: y_res,
                    };
                } else {
                    let s = (x.pow(2).scale(3) + a) / (y.scale(2));
                    let x_res = s.pow(2) - x.scale(2);
                    let y_res = s * (x - x_res) - y;
                    return Point::Coor {
                        a,
                        b,
                        x: x_res,
                        y: y_res,
                    };
                }
            }
            _ => panic!("unrecognized case"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = FiniteField::new(0, prime);
        let b = FiniteField::new(7, prime);

        // on the curve
        let x = FiniteField::new(192, prime);
        let y = FiniteField::new(105, prime);

        assert!(Point::is_on_curve(&Point::Coor { a, b, x, y }));

        let x = FiniteField::new(17, prime);
        let y = FiniteField::new(56, prime);

        assert!(Point::is_on_curve(&Point::Coor { a, b, x, y }));

        let x = FiniteField::new(1, prime);
        let y = FiniteField::new(193, prime);

        assert!(Point::is_on_curve(&Point::Coor { a, b, x, y }));

        // not on the curve
        let x = FiniteField::new(200, prime);
        let y = FiniteField::new(119, prime);

        assert!(!Point::is_on_curve(&Point::Coor { a, b, x, y }));

        let x = FiniteField::new(42, prime);
        let y = FiniteField::new(99, prime);

        assert!(!Point::is_on_curve(&Point::Coor { a, b, x, y }));
    }

    #[test]
    fn test_point_addition() {
        let prime = 223;
        let a = FiniteField::new(0, prime);
        let b = FiniteField::new(7, prime);

        let x = FiniteField::new(192, prime);
        let y = FiniteField::new(105, prime);

        let p1 = Point::new(a, b, x, y);

        let x = FiniteField::new(17, prime);
        let y = FiniteField::new(56, prime);

        let p2 = Point::new(a, b, x, y);

        let x = FiniteField::new(170, prime);
        let y = FiniteField::new(142, prime);

        let p3 = Point::new(a, b, x, y);

        assert_eq!(p1 + p2, p3);

        // (170,142) + (60, 139)
        let x = FiniteField::new(170, prime);
        let y = FiniteField::new(142, prime);

        let p1 = Point::new(a, b, x, y);

        let x = FiniteField::new(60, prime);
        let y = FiniteField::new(139, prime);

        let p2 = Point::new(a, b, x, y);

        let x = FiniteField::new(220, prime);
        let y = FiniteField::new(181, prime);

        let p3 = Point::new(a, b, x, y);

        assert_eq!(p1 + p2, p3);

        // (47,71) + (17,56)
        let x = FiniteField::new(47, prime);
        let y = FiniteField::new(71, prime);

        let p1 = Point::new(a, b, x, y);

        let x = FiniteField::new(17, prime);
        let y = FiniteField::new(56, prime);

        let p2 = Point::new(a, b, x, y);

        let x = FiniteField::new(215, prime);
        let y = FiniteField::new(68, prime);

        let p3 = Point::new(a, b, x, y);

        assert_eq!(p1 + p2, p3);

        // (143,98) + (76,66)
        let x = FiniteField::new(143, prime);
        let y = FiniteField::new(98, prime);

        let p1 = Point::new(a, b, x, y);

        let x = FiniteField::new(76, prime);
        let y = FiniteField::new(66, prime);

        let p2 = Point::new(a, b, x, y);

        let x = FiniteField::new(47, prime);
        let y = FiniteField::new(71, prime);

        let p3 = Point::new(a, b, x, y);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_scale() {
        let prime = 223;
        let a = FiniteField::new(0, prime);
        let b = FiniteField::new(7, prime);

        let x = FiniteField::new(47, prime);
        let y = FiniteField::new(71, prime);

        let p = Point::new(a, b, x, y);

        let x = FiniteField::new(47, prime);
        let y = FiniteField::new(71, prime);
        let pr = Point::new(a, b, x, y);
        assert_eq!(p.scale(1), pr);

        let x = FiniteField::new(36, prime);
        let y = FiniteField::new(111, prime);
        let pr = Point::new(a, b, x, y);
        assert_eq!(p.scale(2), pr);

        let x = FiniteField::new(15, prime);
        let y = FiniteField::new(137, prime);
        let pr = Point::new(a, b, x, y);
        assert_eq!(p.scale(3), pr);

        let x = FiniteField::new(194, prime);
        let y = FiniteField::new(51, prime);
        let pr = Point::new(a, b, x, y);
        assert_eq!(p.scale(4), pr);
    }
}
