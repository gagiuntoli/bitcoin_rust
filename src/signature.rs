#![allow(dead_code)]

use std::fmt::Debug;

use crate::point::Point;
use crate::secp256k1::Secp256k1Point;
use hmac::{Hmac, Mac};
use num::One;
use num_bigint::BigUint;
use sha2::Sha256;

pub type PublicKey = Secp256k1Point; // P = e * G

#[derive(Debug)]
pub struct Signature {
    pub r: BigUint,
    pub s: BigUint,
}

impl Signature {
    pub fn sign(z: &[u8], e: &BigUint, k: &BigUint) -> Signature {
        let z = BigUint::from_bytes_be(z);
        let point = Secp256k1Point::generator().scale(k.clone());

        if let Point::Coor { x, .. } = point {
            let r = x.number;
            let k_inv = k.modpow(&Secp256k1Point::n_minus_2(), &Secp256k1Point::n());
            let s = ((z + r.clone() * e) * k_inv) % Secp256k1Point::n();
            Signature { r, s }
        } else {
            panic!("it was not posible to generate the random point");
        }
    }

    pub fn sign2(z: &[u8], e: &BigUint) -> Signature {
        let k = Self::deterministic_k(z, e, &Secp256k1Point::n());
        let z = BigUint::from_bytes_be(z);
        let point = Secp256k1Point::generator().scale(k.clone());

        if let Point::Coor { x, .. } = point {
            let r = x.number;
            let k_inv = k.modpow(&Secp256k1Point::n_minus_2(), &Secp256k1Point::n());
            let s = ((z + r.clone() * e) * k_inv) % Secp256k1Point::n();
            Signature { r, s }
        } else {
            panic!("it was not posible to generate the random point");
        }
    }

    fn to_bytes32_be(v: &[u8]) -> [u8; 32] {
        let diff = 32 - v.len();
        assert!(diff >= 0);

        let mut buffer = [0u8; 32];
        buffer[diff..].copy_from_slice(&v);
        buffer
    }

    pub fn deterministic_k(z: &[u8], e: &BigUint, n: &BigUint) -> BigUint {
        let k = [0x00u8; 32];
        let v = [0x01u8; 32];
        println!("v = {:?}", v);
        let mut z = BigUint::from_bytes_be(z);

        println!("z1 = {}", hex::encode(z.to_bytes_be()));
        if z > n.clone() {
            z -= n;
        }
        println!("z1 = {}", hex::encode(z.to_bytes_be()));

        let z_bytes = Self::to_bytes32_be(&z.to_bytes_be());
        let e_bytes = Self::to_bytes32_be(&e.to_bytes_be());

        // Create alias for HMAC-SHA256
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
        let msg = [&v[..], &[0u8; 1][..], &e_bytes[..], &z_bytes[..]].concat();
        mac.update(&msg);
        let k = mac.finalize().into_bytes();
        assert_eq!(k.len(), 32);

        let mut mac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
        let msg = v;
        mac.update(&msg);
        let v = mac.finalize().into_bytes();
        assert_eq!(v.len(), 32);

        println!("k = {:x?}", k);
        println!("v = {:x?}", v);

        let mut mac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
        let msg = [&v[..], &[0u8; 1][..], &e_bytes[..], &z_bytes[..]].concat();
        mac.update(&msg);
        let k = mac.finalize().into_bytes();
        assert_eq!(k.len(), 32);

        let mut mac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
        let msg = v;
        mac.update(&msg);
        let v = mac.finalize().into_bytes();
        assert_eq!(v.len(), 32);

        println!("k = {:0x?}", k);
        println!("v = {:0x?}", v);

        loop {
            let mut mac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
            let msg = v;
            mac.update(&msg);
            let v = mac.finalize().into_bytes();
            assert_eq!(v.len(), 32);

            let candidate = BigUint::from_bytes_be(&v);
            if candidate > BigUint::one() && candidate < Secp256k1Point::n() {
                return candidate;
            }

            let mut mac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
            let msg = [&v[..], &[0u8; 1][..]].concat();
            mac.update(&msg);
            let k = mac.finalize().into_bytes();
            assert_eq!(k.len(), 32);

            let mut mac = HmacSha256::new_from_slice(&k).expect("HMAC can take key of any size");
            let msg = v;
            mac.update(&msg);
            let v = mac.finalize().into_bytes();
            assert_eq!(v.len(), 32);
        }

        BigUint::from(12345u32)
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
    use crate::hash::sha256_double;
    use hex;

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

    #[test]
    fn test_verification_exercise_6() {
        let px = hex::decode("887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c")
            .unwrap();
        let py = hex::decode("61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34")
            .unwrap();
        let public_key = Secp256k1Point::from_bytes_be(&px, &py);

        let z = hex::decode("ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60")
            .unwrap();
        let r = hex::decode("ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395")
            .unwrap();
        let s = hex::decode("068342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4")
            .unwrap();

        let signature = Signature {
            r: BigUint::from_bytes_be(&r),
            s: BigUint::from_bytes_be(&s),
        };

        let result = Signature::verify(&signature, &z, &public_key);

        assert!(result);

        let z = hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d")
            .unwrap();
        let r =
            hex::decode("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap();
        let s = hex::decode("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6")
            .unwrap();

        let signature = Signature {
            r: BigUint::from_bytes_be(&r),
            s: BigUint::from_bytes_be(&s),
        };

        let result = Signature::verify(&signature, &z, &public_key);

        assert!(result);
    }

    #[test]
    fn test_sign() {
        let e = BigUint::from_bytes_be(&sha256_double("my secret"));
        let z = sha256_double("my message");
        let k = BigUint::from(1234567890u32);

        let signature = Signature::sign(&z, &e, &k);

        let public_key = Secp256k1Point::compute_public_key(&e);

        if let Point::Coor { x, y, .. } = public_key {
            assert_eq!(
                hex::encode(x.number.to_bytes_be()),
                "028d003eab2e428d11983f3e97c3fa0addf3b42740df0d211795ffb3be2f6c52"
            );
            assert_eq!(
                hex::encode(y.number.to_bytes_be()),
                "0ae987b9ec6ea159c78cb2a937ed89096fb218d9e7594f02b547526d8cd309e2"
            );
        }

        assert_eq!(
            hex::encode(signature.r.to_bytes_be()),
            "2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22"
        );
        assert_eq!(
            hex::encode(signature.s.to_bytes_be()),
            "bb14e602ef9e3f872e25fad328466b34e6734b7a0fcd58b1eb635447ffae8cb9"
        );
    }

    #[test]
    fn test_sign_2() {
        let e = BigUint::from(12345u32);

        let public_key = Secp256k1Point::compute_public_key(&e);

        if let Point::Coor { x, y, .. } = public_key {
            assert_eq!(
                hex::encode(x.number.to_bytes_be()),
                "f01d6b9018ab421dd410404cb869072065522bf85734008f105cf385a023a80f"
            );
            assert_eq!(
                hex::encode(y.number.to_bytes_be()),
                "0eba29d0f0c5408ed681984dc525982abefccd9f7ff01dd26da4999cf3f6a295"
            );
        }

        let z = sha256_double("Programming Bitcoin!");

        let k = BigUint::from(1234567890u32);

        let signature = Signature::sign(&z, &e, &k);

        assert_eq!(
            hex::encode(z),
            "969f6056aa26f7d2795fd013fe88868d09c9f6aed96965016e1936ae47060d48"
        );

        assert_eq!(
            hex::encode(signature.r.to_bytes_be()),
            "2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22"
        );
        assert_eq!(
            hex::encode(signature.s.to_bytes_be()),
            "1dbc63bfef4416705e602a7b564161167076d8b20990a0f26f316cff2cb0bc1a"
        );
    }

    #[test]
    fn test_sign_deterministic_k() {
        let e = BigUint::from(12345u32);

        let z = sha256_double("Programming Bitcoin!");

        let signature = Signature::sign2(&z, &e);
    }

    #[test]
    fn test_deterministic_k() {
        // https://www.rfc-editor.org/rfc/rfc6979
        let q = BigUint::from_bytes_be(
            &hex::decode("04000000000000000000020108a2e0cc0d99f8a5ef").unwrap(),
        );

        let e = BigUint::from_bytes_be(
            &hex::decode("009a4d6792295a7f730fc3f2b49cbc0f62e862272f").unwrap(),
        );

        let z = &hex::decode("af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d0268e9891562113d8a62add1bf")
            .unwrap();

        let k = Signature::deterministic_k(z, &e, &q);
        println!("k final = {}", hex::encode(k.to_bytes_be()));
    }
}
