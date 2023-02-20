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
        if !Self::is_in_curve(&point) {
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

    fn is_in_curve(p: &Point) -> bool {
        match p {
            Point::Coor { a, b, x, y } => return y.pow(2) == x.pow(3) + (*a) * (*x) + *b,
            Point::Zero => true,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        if self.is_zero() {
            return rhs;
        } else if rhs.is_zero() {
            return self;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_in_curve() {
        let a = FiniteField::new(5, 1000);
        let b = FiniteField::new(7, 1000);

        let x = FiniteField::new(2, 1000);
        let y = FiniteField::new(4, 1000);

        assert!(!Point::is_in_curve(&Point::Coor { a, b, x, y }));

        let x = FiniteField::new(18, 1000);
        let y = FiniteField::new(77, 1000);

        assert!(Point::is_in_curve(&Point::Coor { a, b, x, y }));

        let x = FiniteField::new(5, 1000);
        let y = FiniteField::new(7, 1000);

        assert!(!Point::is_in_curve(&Point::Coor { a, b, x, y }));
    }

    #[test]
    fn test_add_inf() {
        let a = FiniteField::new(5, 1000);
        let b = FiniteField::new(7, 1000);

        let x = FiniteField::new(18, 1000);
        let y = FiniteField::new(77, 1000);

        let point = Point::new(a, b, x, y);

        assert_eq!(point + Point::Zero, point);
        assert_eq!(Point::Zero + point, point);
    }
}
