use crate::finite_field::FiniteField;
use crate::point::Point;

#[allow(unused_imports)]
use num_bigint::BigUint;

struct Secp256k1Point;

impl Secp256k1Point {
    #[allow(dead_code)]
    fn zero() -> Point {
        Point::Zero
    }

    #[allow(dead_code)]
    fn new(x: &[u8], y: &[u8]) -> Point {
        let prime = hex::decode("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
            .unwrap();

        let a = hex::decode("00").unwrap();
        let b = hex::decode("07").unwrap();

        let a = FiniteField::from_bytes_be(&a, &prime);
        let b = FiniteField::from_bytes_be(&b, &prime);
        let x = FiniteField::from_bytes_be(&x, &prime);
        let y = FiniteField::from_bytes_be(&y, &prime);

        let point = Point::Coor {
            a,
            b,
            x: x.clone(),
            y: y.clone(),
        };

        if !Point::is_on_curve(&point) {
            panic!("({:?},{:?}) point is not in the curve", x, y);
        }

        point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_bitcoin_generator_point() {
        let gx = hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
            .unwrap();
        let gy = hex::decode("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8")
            .unwrap();

        let point = Secp256k1Point::new(&gx, &gy);

        assert!(Point::is_on_curve(&point));

        let n = hex::decode("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141")
            .unwrap();

        assert_eq!(point.scale(BigUint::from_bytes_be(&n)), Point::Zero);
    }
}
