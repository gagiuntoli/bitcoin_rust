use num::{Integer, One};
use num_bigint::{BigInt, BigUint, ToBigInt};
use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(PartialEq, Debug, Clone)]
pub struct FiniteField {
    number: BigUint,
    prime: BigUint,
}

impl FiniteField {
    #[allow(dead_code)]
    pub fn new(number: BigUint, prime: BigUint) -> Self {
        if number >= prime {
            panic!(
                "Number: {} isn't in the range [0, prime = {})",
                number, prime
            );
        }
        FiniteField { number, prime }
    }

    #[allow(dead_code)]
    pub fn new_from_u32(number: u32, prime: u32) -> Self {
        if number >= prime {
            panic!(
                "Number: {} isn't in the range [0, prime = {})",
                number, prime
            );
        }

        let number = BigUint::from(number);
        let prime = BigUint::from(prime);

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

    pub fn pow(self, exp: BigInt) -> FiniteField {
        let exp = exp.mod_floor(&(self.prime.clone() - BigUint::one()).to_bigint().unwrap());
        let exp = exp.to_biguint().unwrap();

        let exp = exp.modpow(
            &BigUint::from_str("1").unwrap(),
            &(self.prime.clone() - BigUint::one()),
        );

        FiniteField {
            number: self.number.modpow(&exp, &self.prime),
            prime: self.prime,
        }
    }

    #[allow(dead_code)]
    pub fn scale(self, scalar: BigUint) -> FiniteField {
        FiniteField {
            number: (self.number * scalar) % self.prime.clone(),
            prime: self.prime,
        }
    }
}

impl Add for FiniteField {
    type Output = FiniteField;

    fn add(self, _rhs: FiniteField) -> FiniteField {
        self.check_equal_order_and_panic(&_rhs);

        FiniteField {
            number: (self.number + _rhs.number) % self.prime.clone(),
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
                number: (self.number - rhs.number) % self.prime.clone(),
                prime: self.prime,
            }
        } else {
            FiniteField {
                number: (self.number + self.prime.clone() - rhs.number) % self.prime.clone(),
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
            number: (self.number * rhs.number) % self.prime.clone(),
            prime: self.prime,
        }
    }
}

impl Div for FiniteField {
    type Output = FiniteField;

    fn div(self, rhs: FiniteField) -> FiniteField {
        self.check_equal_order_and_panic(&rhs);

        self.clone() * rhs.pow((self.prime - BigUint::from(2u32)).to_bigint().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_and_non_equal() {
        // a != b
        // b == c => a != c
        let prime = BigUint::from(11u32);
        let a = FiniteField::new(BigUint::from(6u32), prime.clone());
        let b = FiniteField::new(BigUint::from(5u32), prime.clone());
        let c = FiniteField::new(BigUint::from(5u32), prime.clone());

        assert_ne!(a, b);
        assert_eq!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = FiniteField::new(BigUint::from(6u32), BigUint::from(11u32));
        let b = FiniteField::new(BigUint::from(5u32), BigUint::from(11u32));
        let c = FiniteField::new(BigUint::from(0u32), BigUint::from(11u32));

        assert_eq!(a + b, c);

        let a = FiniteField::new(BigUint::from(44u32), BigUint::from(57u32));
        let b = FiniteField::new(BigUint::from(33u32), BigUint::from(57u32));
        let c = FiniteField::new(BigUint::from(20u32), BigUint::from(57u32));

        assert_eq!(a + b, c);

        let a = FiniteField::new(BigUint::from(17u32), BigUint::from(57u32));
        let b = FiniteField::new(BigUint::from(42u32), BigUint::from(57u32));
        let c = FiniteField::new(BigUint::from(49u32), BigUint::from(57u32));
        let d = FiniteField::new(BigUint::from(51u32), BigUint::from(57u32));

        assert_eq!(a + b + c, d);
    }

    #[test]
    #[should_panic(expected = "Finite fields elements have different order lhs: 12, rhs: 11")]
    fn test_fail_adding_different_orders() {
        let a = FiniteField::new(BigUint::from(6u32), BigUint::from(12u32));
        let b = FiniteField::new(BigUint::from(5u32), BigUint::from(11u32));

        let _c = a + b;
    }

    #[test]
    fn test_substract() {
        let a = FiniteField::new(BigUint::from(6u32), BigUint::from(11u32));
        let b = FiniteField::new(BigUint::from(5u32), BigUint::from(11u32));
        let c = FiniteField::new(BigUint::from(1u32), BigUint::from(11u32));

        assert_eq!(a - b, c);

        let a = FiniteField::new(BigUint::from(52u32), BigUint::from(57u32));
        let b = FiniteField::new(BigUint::from(30u32), BigUint::from(57u32));
        let c = FiniteField::new(BigUint::from(38u32), BigUint::from(57u32));
        let d = FiniteField::new(BigUint::from(41u32), BigUint::from(57u32));

        assert_eq!(a - b - c, d);
    }

    #[test]
    #[should_panic(expected = "Finite fields elements have different order lhs: 12, rhs: 11")]
    fn test_fail_substracting_different_orders() {
        let a = FiniteField::new(BigUint::from(6u32), BigUint::from(12u32));
        let b = FiniteField::new(BigUint::from(5u32), BigUint::from(11u32));

        let _c = a - b;
    }

    #[test]
    fn test_multiply() {
        let a = FiniteField::new(BigUint::from(6u32), BigUint::from(11u32));
        let b = FiniteField::new(BigUint::from(5u32), BigUint::from(11u32));
        let c = FiniteField::new(BigUint::from(8u32), BigUint::from(11u32));

        assert_eq!(a * b, c);
    }

    #[test]
    #[should_panic(expected = "Finite fields elements have different order lhs: 12, rhs: 11")]
    fn test_fail_multiplying_different_orders() {
        let a = FiniteField::new(BigUint::from(6u32), BigUint::from(12u32));
        let b = FiniteField::new(BigUint::from(5u32), BigUint::from(11u32));

        let _c = a * b;
    }

    #[test]
    fn test_exponentiation() {
        let a = FiniteField::new(BigUint::from(7u32), BigUint::from(19u32));
        let b = FiniteField::new(BigUint::from(1u32), BigUint::from(19u32));

        assert_eq!(a.pow(BigInt::from(3u32)), b);

        let a = FiniteField::new(BigUint::from(9u32), BigUint::from(19u32));
        let b = FiniteField::new(BigUint::from(7u32), BigUint::from(19u32));

        assert_eq!(a.pow(BigInt::from(12u32)), b);

        let a = FiniteField::new(BigUint::from(12u32), BigUint::from(97u32));
        let b = FiniteField::new(BigUint::from(77u32), BigUint::from(97u32));
        let c = FiniteField::new(BigUint::from(63u32), BigUint::from(97u32));

        assert_eq!(a.pow(BigInt::from(7u32)) * b.pow(BigInt::from(49u32)), c);
    }

    #[test]
    fn test_exercise_5() {
        let all_elements = (0..19)
            .map(|i| FiniteField::new(BigUint::from(i as u32), BigUint::from(19u32)))
            .collect::<Vec<FiniteField>>();

        assert!((0..19)
            .map(|i| FiniteField::new(BigUint::from(i as u32 * 1), BigUint::from(19u32)))
            .all(|elem| all_elements.contains(&elem)));

        assert!((0..19)
            .map(|i| FiniteField::new(BigUint::from((i as u32 * 3) % 19), BigUint::from(19u32)))
            .all(|elem| all_elements.contains(&elem)));

        assert!((0..19)
            .map(|i| FiniteField::new(BigUint::from((i as u32 * 7) % 19), BigUint::from(19u32)))
            .all(|elem| all_elements.contains(&elem)));

        assert!((0..19)
            .map(|i| FiniteField::new(BigUint::from((i as u32 * 13) % 19), BigUint::from(19u32)))
            .all(|elem| all_elements.contains(&elem)));

        assert!((0..19)
            .map(|i| FiniteField::new(BigUint::from((i as u32 * 18) % 19), BigUint::from(19u32)))
            .all(|elem| all_elements.contains(&elem)));
    }

    #[test]
    fn test_exercise_7() {
        assert!((1..7)
            .map(
                |i| FiniteField::new(BigUint::from(i as u32), BigUint::from(7u32))
                    .pow(BigInt::from(7u32 - 1))
            )
            .all(|elem| elem == FiniteField::new(BigUint::one(), BigUint::from(7u32))));

        assert!((1..11)
            .map(
                |i| FiniteField::new(BigUint::from(i as u32), BigUint::from(11u32))
                    .pow(BigInt::from(11u32 - 1))
            )
            .all(|elem| elem == FiniteField::new(BigUint::one(), BigUint::from(11u32))));

        assert!((1..17)
            .map(
                |i| FiniteField::new(BigUint::from(i as u32), BigUint::from(17u32))
                    .pow(BigInt::from(17u32 - 1))
            )
            .all(|elem| elem == FiniteField::new(BigUint::one(), BigUint::from(17u32))));

        assert!((1..31)
            .map(
                |i| FiniteField::new(BigUint::from(i as u32), BigUint::from(31u32))
                    .pow(BigInt::from(31u32 - 1))
            )
            .all(|elem| elem == FiniteField::new(BigUint::one(), BigUint::from(31u32))));
    }

    #[test]
    fn test_operations() {
        let a = FiniteField::new(BigUint::from(3u32), BigUint::from(31u32));
        let b = FiniteField::new(BigUint::from(24u32), BigUint::from(31u32));
        let c = FiniteField::new(BigUint::from(4u32), BigUint::from(31u32));

        assert_eq!(a / b, c);

        let a = FiniteField::new(BigUint::from(17u32), BigUint::from(31u32));
        let b = FiniteField::new(BigUint::from(29u32), BigUint::from(31u32));

        assert_eq!(a.pow(BigInt::from(-3)), b);

        let a = FiniteField::new(BigUint::from(4u32), BigUint::from(31u32));
        let b = FiniteField::new(BigUint::from(11u32), BigUint::from(31u32));
        let c = FiniteField::new(BigUint::from(13u32), BigUint::from(31u32));

        assert_eq!(a.pow(BigInt::from(-4)) * b, c);
    }
}
