use crate::finite_field::FiniteField;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point {
    a: FiniteField,
    b: FiniteField,
    x: Option<FiniteField>,
    y: Option<FiniteField>,
}

impl Point {
    fn new(
        a: FiniteField,
        b: FiniteField,
        x: Option<FiniteField>,
        y: Option<FiniteField>,
    ) -> Point {
        let point = Point { a, b, x, y };
        if !Self::is_in_curve(&point) {
            panic!("({:?},{:?}) point is not in the curve", x, y);
        }
        point
    }

    fn inf(a: FiniteField, b: FiniteField) -> Point {
        Point {
            a,
            b,
            x: None,
            y: None,
        }
    }

    fn is_in_curve(p: &Point) -> bool {
        match (p.x, p.y) {
            (Some(x), Some(y)) => return y.pow(2) == x.pow(3) + p.a * x + p.b,
            (None, _) => true,
            (_, None) => true,
        }
    }

    fn is_point_in_inf(&self) -> bool {
        self.x == None
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        if self.is_point_in_inf() {
            return rhs;
        } else if rhs.is_point_in_inf() {
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

        let x = Some(FiniteField::new(2, 1000));
        let y = Some(FiniteField::new(4, 1000));

        assert!(!Point::is_in_curve(&Point { a, b, x, y }));

        let x = Some(FiniteField::new(18, 1000));
        let y = Some(FiniteField::new(77, 1000));

        assert!(Point::is_in_curve(&Point { a, b, x, y }));

        let x = Some(FiniteField::new(5, 1000));
        let y = Some(FiniteField::new(7, 1000));

        assert!(!Point::is_in_curve(&Point { a, b, x, y }));
    }

    #[test]
    fn test_add_inf() {
        let a = FiniteField::new(5, 1000);
        let b = FiniteField::new(7, 1000);

        let x = Some(FiniteField::new(18, 1000));
        let y = Some(FiniteField::new(77, 1000));

        let point_inf = Point::inf(a, b);
        let point = Point::new(a, b, x, y);

        assert_eq!(point + point_inf, point);
        assert_eq!(point_inf + point, point);
    }
}
