use crate::finite_field::FiniteField;
use num_bigint::{BigInt, BigUint};
use std::ops::Add;

#[derive(PartialEq, Debug, Clone)]
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
    #[allow(dead_code)]
    fn new(a: FiniteField, b: FiniteField, x: FiniteField, y: FiniteField) -> Point {
        let point = Point::Coor {
            a: a.clone(),
            b: b.clone(),
            x: x.clone(),
            y: y.clone(),
        };
        if !Self::is_on_curve(&point) {
            panic!("({:?},{:?}) point is not in the curve", x, y);
        }
        point
    }

    #[allow(dead_code)]
    fn zero() -> Self {
        Point::Zero
    }

    #[allow(dead_code)]
    fn is_zero(self) -> bool {
        self == Point::Zero
    }

    fn is_on_curve(p: &Point) -> bool {
        match p {
            Point::Coor { a, b, x, y } => {
                return y.clone().pow(BigInt::from(2u32))
                    == x.clone().pow(BigInt::from(3u32)) + a.clone() * x.clone() + b.clone()
            }
            Point::Zero => true,
        }
    }

    #[allow(dead_code)]
    fn scale(self, _scalar: u32) -> Self {
        let mut current = self.clone();
        let mut result = Point::Zero;
        let mut scalar = _scalar;

        while scalar != 0 {
            if scalar & 1 != 0 {
                result = result + current.clone();
            }
            current = current.clone() + current;
            scalar = scalar >> 1;
        }
        return result;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        match (self.clone(), rhs.clone()) {
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
                    let s = (y_rhs.clone() - y.clone()) / (x_rhs.clone() - x.clone());
                    let x_res = s.clone().pow(BigInt::from(2u32)) - x.clone() - x_rhs.clone();
                    let y_res = s.clone() * (x.clone() - x_res.clone()) - y;
                    return Point::Coor {
                        a,
                        b,
                        x: x_res,
                        y: y_res,
                    };
                } else {
                    let s = (x.clone().pow(BigInt::from(2u32)).scale(BigUint::from(3u32))
                        + a.clone())
                        / (y.clone().scale(BigUint::from(2u32)));
                    let x_res =
                        s.clone().pow(BigInt::from(2u32)) - x.clone().scale(BigUint::from(2u32));
                    let y_res = s * (x - x_res.clone()) - y;
                    return Point::Coor {
                        a,
                        b,
                        x: x_res,
                        y: y_res,
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_curve() {
        let prime = BigUint::from(223u32);
        let a = FiniteField::new(BigUint::from(0u32), prime.clone());
        let b = FiniteField::new(BigUint::from(7u32), prime.clone());

        // on the curve
        let x = FiniteField::new(BigUint::from(192u32), prime.clone());
        let y = FiniteField::new(BigUint::from(105u32), prime.clone());

        assert!(Point::is_on_curve(&Point::Coor {
            a: a.clone(),
            b: b.clone(),
            x,
            y
        }));

        let x = FiniteField::new(BigUint::from(17u32), prime.clone());
        let y = FiniteField::new(BigUint::from(56u32), prime.clone());

        assert!(Point::is_on_curve(&Point::Coor {
            a: a.clone(),
            b: b.clone(),
            x,
            y
        }));

        let x = FiniteField::new(BigUint::from(1u32), prime.clone());
        let y = FiniteField::new(BigUint::from(193u32), prime.clone());

        assert!(Point::is_on_curve(&Point::Coor {
            a: a.clone(),
            b: b.clone(),
            x,
            y
        }));

        // not on the curve
        let x = FiniteField::new(BigUint::from(200u32), prime.clone());
        let y = FiniteField::new(BigUint::from(119u32), prime.clone());

        assert!(!Point::is_on_curve(&Point::Coor {
            a: a.clone(),
            b: b.clone(),
            x,
            y
        }));

        let x = FiniteField::new(BigUint::from(42u32), prime.clone());
        let y = FiniteField::new(BigUint::from(99u32), prime.clone());

        assert!(!Point::is_on_curve(&Point::Coor { a, b, x, y }));
    }

    #[test]
    fn test_point_addition() {
        let prime = BigUint::from(223u32);
        let a = FiniteField::new(BigUint::from(0u32), prime.clone());
        let b = FiniteField::new(BigUint::from(7u32), prime.clone());

        let x = FiniteField::new(BigUint::from(192u32), prime.clone());
        let y = FiniteField::new(BigUint::from(105u32), prime.clone());

        let p1 = Point::new(a.clone(), b.clone(), x, y);

        let x = FiniteField::new(BigUint::from(17u32), prime.clone());
        let y = FiniteField::new(BigUint::from(56u32), prime.clone());

        let p2 = Point::new(a.clone(), b.clone(), x, y);

        let x = FiniteField::new(BigUint::from(170u32), prime.clone());
        let y = FiniteField::new(BigUint::from(142u32), prime.clone());

        let p3 = Point::new(a.clone(), b.clone(), x, y);

        assert_eq!(p1 + p2, p3);

        // (170,142) + (60, 139)
        let x = FiniteField::new(BigUint::from(170u32), prime.clone());
        let y = FiniteField::new(BigUint::from(142u32), prime.clone());

        let p1 = Point::new(a.clone(), b.clone(), x, y);

        let x = FiniteField::new(BigUint::from(60u32), prime.clone());
        let y = FiniteField::new(BigUint::from(139u32), prime.clone());

        let p2 = Point::new(a.clone(), b.clone(), x, y);

        let x = FiniteField::new(BigUint::from(220u32), prime.clone());
        let y = FiniteField::new(BigUint::from(181u32), prime.clone());

        let p3 = Point::new(a.clone(), b.clone(), x, y);

        assert_eq!(p1 + p2, p3);

        // (47,71) + (17,56)
        let x = FiniteField::new(BigUint::from(47u32), prime.clone());
        let y = FiniteField::new(BigUint::from(71u32), prime.clone());

        let p1 = Point::new(a.clone(), b.clone(), x, y);

        let x = FiniteField::new(BigUint::from(17u32), prime.clone());
        let y = FiniteField::new(BigUint::from(56u32), prime.clone());

        let p2 = Point::new(a.clone(), b.clone(), x, y);

        let x = FiniteField::new(BigUint::from(215u32), prime.clone());
        let y = FiniteField::new(BigUint::from(68u32), prime.clone());

        let p3 = Point::new(a.clone(), b.clone(), x, y);

        assert_eq!(p1 + p2, p3);

        // (143,98) + (76,66)
        let x = FiniteField::new(BigUint::from(143u32), prime.clone());
        let y = FiniteField::new(BigUint::from(98u32), prime.clone());

        let p1 = Point::new(a.clone(), b.clone(), x, y);

        let x = FiniteField::new(BigUint::from(76u32), prime.clone());
        let y = FiniteField::new(BigUint::from(66u32), prime.clone());

        let p2 = Point::new(a.clone(), b.clone(), x, y);

        let x = FiniteField::new(BigUint::from(47u32), prime.clone());
        let y = FiniteField::new(BigUint::from(71u32), prime.clone());

        let p3 = Point::new(a, b, x, y);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_scale() {
        let prime = BigUint::from(223u32);
        let a = FiniteField::new(BigUint::from(0u32), prime.clone());
        let b = FiniteField::new(BigUint::from(7u32), prime.clone());

        let x = FiniteField::new(BigUint::from(47u32), prime.clone());
        let y = FiniteField::new(BigUint::from(71u32), prime.clone());

        let p = Point::new(a.clone(), b.clone(), x, y);

        let x = FiniteField::new(BigUint::from(47u32), prime.clone());
        let y = FiniteField::new(BigUint::from(71u32), prime.clone());
        let pr = Point::new(a.clone(), b.clone(), x, y);
        assert_eq!(p.clone().scale(1), pr);

        let x = FiniteField::new(BigUint::from(36u32), prime.clone());
        let y = FiniteField::new(BigUint::from(111u32), prime.clone());
        let pr = Point::new(a.clone(), b.clone(), x, y);
        assert_eq!(p.clone().scale(2), pr);

        let x = FiniteField::new(BigUint::from(15u32), prime.clone());
        let y = FiniteField::new(BigUint::from(137u32), prime.clone());
        let pr = Point::new(a.clone(), b.clone(), x, y);
        assert_eq!(p.clone().scale(3), pr);

        let x = FiniteField::new(BigUint::from(194u32), prime.clone());
        let y = FiniteField::new(BigUint::from(51u32), prime.clone());
        let pr = Point::new(a.clone(), b.clone(), x, y);
        assert_eq!(p.clone().scale(4), pr);

        let x = FiniteField::new(BigUint::from(47u32), prime.clone());
        let y = FiniteField::new(BigUint::from(152u32), prime);
        let pr = Point::new(a, b, x, y);
        assert_eq!(p.scale(20), pr);
    }
}
