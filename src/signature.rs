use crate::point::Point;
use crate::secp256k1::Secp256k1Point;
use num_bigint::BigUint;

type PublicKey = Secp256k1Point; // P = e * G

struct Signature {
    r: BigUint,
    s: BigUint,
}

impl Signature {
    pub fn sign(z: &[u8], public_key: &PublicKey) -> Self {
        todo!()
    }

    pub fn verify(signature: &Signature, z: &[u8], public_key: &PublicKey) -> bool {
        let n = Secp256k1Point::n();

        let s_inv = signature
            .s
            .clone()
            .modpow(&Secp256k1Point::n_minus_2(), &Secp256k1Point::n());
        let z = BigUint::from_bytes_be(z);
        let r = &signature.r;

        // (z * s_inv) is always positive so remainder and module are the same
        let u = (z * s_inv.clone()) % n.clone();
        let v = (r * s_inv) % n.clone();

        let generator = Secp256k1Point::generator();
        let point = generator.scale(u) + public_key.clone().scale(v);

        if let Point::Coor { x, .. } = point {
            x.number == *r
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use num_bigint::Sign;

    #[test]
    fn test_verification_true() {
        let z = hex::decode("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423")
            .unwrap();
        let r = hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6")
            .unwrap();
        let s = hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec")
            .unwrap();
        let px = hex::decode("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574")
            .unwrap();
        let py = hex::decode("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4")
            .unwrap();

        let signature = Signature {
            r: BigUint::from_bytes_be(&r),
            s: BigUint::from_bytes_be(&s),
        };

        let public_key = Secp256k1Point::from_bytes_be(&px, &py);

        let result = Signature::verify(&signature, &z, &public_key);

        assert!(result);
    }

    #[test]
    fn test_verification_false_altered_message_hash_z() {
        let z = hex::decode("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74424")
            .unwrap();
        let r = hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6")
            .unwrap();
        let s = hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec")
            .unwrap();
        let px = hex::decode("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574")
            .unwrap();
        let py = hex::decode("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4")
            .unwrap();

        let signature = Signature {
            r: BigUint::from_bytes_be(&r),
            s: BigUint::from_bytes_be(&s),
        };

        let public_key = Secp256k1Point::from_bytes_be(&px, &py);

        let result = Signature::verify(&signature, &z, &public_key);

        assert!(!result);
    }

    #[test]
    fn test_verification_false_altered_signature() {
        let z = hex::decode("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423")
            .unwrap();
        let r = hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c7")
            .unwrap();
        let s = hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec")
            .unwrap();
        let px = hex::decode("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574")
            .unwrap();
        let py = hex::decode("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4")
            .unwrap();

        let signature = Signature {
            r: BigUint::from_bytes_be(&r),
            s: BigUint::from_bytes_be(&s),
        };

        let public_key = Secp256k1Point::from_bytes_be(&px, &py);

        let result = Signature::verify(&signature, &z, &public_key);

        assert!(!result);
    }

    #[test]
    fn test_verification_false_altered_public_key() {
        let z = hex::decode("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423")
            .unwrap();
        let r = hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6")
            .unwrap();
        let s = hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec")
            .unwrap();
        let px = hex::decode("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574")
            .unwrap();
        let py = hex::decode("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4")
            .unwrap();

        let signature = Signature {
            r: BigUint::from_bytes_be(&r),
            s: BigUint::from_bytes_be(&s),
        };

        // we need to create a true public key which is a point on the curve
        let public_key = Secp256k1Point::from_bytes_be(&px, &py).scale(BigUint::from(2u32));

        let result = Signature::verify(&signature, &z, &public_key);

        assert!(!result);
    }
}
