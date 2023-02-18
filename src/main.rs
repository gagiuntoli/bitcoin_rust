use mod_exp::mod_exp;
use std::ops::{Add, Div, Mul, Sub};

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct FiniteField {
    number: u32,
    prime: u32,
}

impl FiniteField {
    fn new(number: u32, prime: u32) -> Self {
        if number >= prime {
            panic!(
                "Number: {} isn't in the range [0, prime = {})",
                number, prime
            );
        }
        FiniteField { number, prime }
    }

    fn check_equal_order_and_panic(self: &Self, rhs: &FiniteField) {
        if self.prime != rhs.prime {
            panic!(
                "Finite fields elements have different order lhs: {}, rhs: {}",
                self.prime, rhs.prime
            )
        }
    }

    fn pow(self, exp: i32) -> FiniteField {
        let exp = Self::module(exp, self.prime as i32 - 1);

        FiniteField {
            number: mod_exp(self.number, exp, self.prime),
            prime: self.prime,
        }
    }

    /**
     * We define the module operation since the % is a remainder
     * Eg:
     * -1 % 2 = -1
     * -1 module 2 = 1
     */
    fn module(a: i32, b: i32) -> u32 {
        (((a % b) + b) % b) as u32
    }
}

impl Add for FiniteField {
    type Output = FiniteField;

    fn add(self, _rhs: FiniteField) -> FiniteField {
        self.check_equal_order_and_panic(&_rhs);

        FiniteField {
            number: (self.number + _rhs.number) % self.prime,
            prime: self.prime,
        }
    }
}

impl Sub for FiniteField {
    type Output = FiniteField;

    fn sub(self, rhs: FiniteField) -> FiniteField {
        self.check_equal_order_and_panic(&rhs);

        if self.number >= rhs.number {
            FiniteField {
                number: (self.number - rhs.number) % self.prime,
                prime: self.prime,
            }
        } else {
            FiniteField {
                number: (self.number + self.prime - rhs.number) % self.prime,
                prime: self.prime,
            }
        }
    }
}

impl Mul for FiniteField {
    type Output = FiniteField;

    fn mul(self, rhs: FiniteField) -> FiniteField {
        self.check_equal_order_and_panic(&rhs);

        FiniteField {
            number: (self.number * rhs.number) % self.prime,
            prime: self.prime,
        }
    }
}

impl Div for FiniteField {
    type Output = FiniteField;

    fn div(self, rhs: FiniteField) -> FiniteField {
        self.check_equal_order_and_panic(&rhs);

        self * rhs.pow(self.prime as i32 - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_and_non_equal() {
        // a != b
        // b == c => a != c
        let a = FiniteField::new(6, 11);
        let b = FiniteField::new(5, 11);
        let c = FiniteField::new(5, 11);

        assert_ne!(a, b);
        assert_eq!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = FiniteField::new(6, 11);
        let b = FiniteField::new(5, 11);
        let c = FiniteField::new(0, 11);

        assert_eq!(a + b, c);

        let a = FiniteField::new(44, 57);
        let b = FiniteField::new(33, 57);
        let c = FiniteField::new(20, 57);

        assert_eq!(a + b, c);

        let a = FiniteField::new(17, 57);
        let b = FiniteField::new(42, 57);
        let c = FiniteField::new(49, 57);
        let d = FiniteField::new(51, 57);

        assert_eq!(a + b + c, d);
    }

    #[test]
    #[should_panic(expected = "Finite fields elements have different order lhs: 12, rhs: 11")]
    fn test_fail_adding_different_orders() {
        let a = FiniteField::new(6, 12);
        let b = FiniteField::new(5, 11);

        let _c = a + b;
    }

    #[test]
    fn test_substract() {
        let a = FiniteField::new(6, 11);
        let b = FiniteField::new(5, 11);
        let c = FiniteField::new(1, 11);

        assert_eq!(a - b, c);

        let a = FiniteField::new(52, 57);
        let b = FiniteField::new(30, 57);
        let c = FiniteField::new(38, 57);
        let d = FiniteField::new(41, 57);

        assert_eq!(a - b - c, d);
    }

    #[test]
    #[should_panic(expected = "Finite fields elements have different order lhs: 12, rhs: 11")]
    fn test_fail_substracting_different_orders() {
        let a = FiniteField::new(6, 12);
        let b = FiniteField::new(5, 11);

        let _c = a - b;
    }

    #[test]
    fn test_multiply() {
        let a = FiniteField::new(6, 11);
        let b = FiniteField::new(5, 11);
        let c = FiniteField::new(8, 11);

        assert_eq!(a * b, c);
    }

    #[test]
    #[should_panic(expected = "Finite fields elements have different order lhs: 12, rhs: 11")]
    fn test_fail_multiplying_different_orders() {
        let a = FiniteField::new(6, 12);
        let b = FiniteField::new(5, 11);

        let _c = a * b;
    }

    #[test]
    fn test_exponentiation() {
        let a = FiniteField::new(7, 19);
        let b = FiniteField::new(1, 19);

        assert_eq!(a.pow(3), b);

        let a = FiniteField::new(9, 19);
        let b = FiniteField::new(7, 19);

        assert_eq!(a.pow(12), b);

        let a = FiniteField::new(12, 97);
        let b = FiniteField::new(77, 97);
        let c = FiniteField::new(63, 97);

        assert_eq!(a.pow(7) * b.pow(49), c);
    }

    #[test]
    fn test_exercise_5() {
        let all_elements = (0..19)
            .map(|i| FiniteField::new(i, 19))
            .collect::<Vec<FiniteField>>();

        assert!((0..19)
            .map(|i| FiniteField::new(i * 1, 19))
            .all(|elem| all_elements.contains(&elem)));

        assert!((0..19)
            .map(|i| FiniteField::new((i * 3) % 19, 19))
            .all(|elem| all_elements.contains(&elem)));

        assert!((0..19)
            .map(|i| FiniteField::new((i * 7) % 19, 19))
            .all(|elem| all_elements.contains(&elem)));

        assert!((0..19)
            .map(|i| FiniteField::new((i * 13) % 19, 19))
            .all(|elem| all_elements.contains(&elem)));

        assert!((0..19)
            .map(|i| FiniteField::new((i * 18) % 19, 19))
            .all(|elem| all_elements.contains(&elem)));
    }

    #[test]
    fn test_exercise_7() {
        assert!((1..7)
            .map(|i| FiniteField::new(i, 7).pow(7 - 1))
            .all(|elem| elem == FiniteField::new(1, 7)));

        assert!((1..11)
            .map(|i| FiniteField::new(i, 11).pow(11 - 1))
            .all(|elem| elem == FiniteField::new(1, 11)));

        assert!((1..17)
            .map(|i| FiniteField::new(i, 17).pow(17 - 1))
            .all(|elem| elem == FiniteField::new(1, 17)));

        assert!((1..31)
            .map(|i| FiniteField::new(i, 31).pow(31 - 1))
            .all(|elem| elem == FiniteField::new(1, 31)));
    }

    #[test]
    fn test_operations() {
        let a = FiniteField::new(3, 31);
        let b = FiniteField::new(24, 31);
        let c = FiniteField::new(4, 31);

        assert_eq!(a / b, c);

        let a = FiniteField::new(17, 31);
        let b = FiniteField::new(29, 31);

        assert_eq!(a.pow(-3), b);

        let a = FiniteField::new(4, 31);
        let b = FiniteField::new(11, 31);
        let c = FiniteField::new(13, 31);

        assert_eq!(a.pow(-4) * b, c);
    }
}
