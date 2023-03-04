#![allow(dead_code)]

use hmac::{Hmac, Mac};
use num::{One, Zero};
use num_bigint::BigUint;
use sha2::Sha256;
use std::cmp::min;

pub fn generate_k<const N: usize, const K: usize>(
    z: &[u8; N],
    e: &[u8; N],
    q: &[u8; N],
) -> BigUint {
    let q_bi = BigUint::from_bytes_be(q);

    let mut k = [0x00; K];
    let mut v = [0x01; K];

    // K = HMAC_K(V || 0x00 || int2octets(x) || bits2octets(h1))
    // V = HMAC_K(V)
    k = hmac(
        &k,
        &[
            &v[..],
            &[0x00][..],
            &int_2_octets::<N>(BigUint::from_bytes_be(e))[..],
            &z[..],
        ]
        .concat(),
    );

    let v = hmac(&k, &v[..]);

    // K = HMAC_K(V || 0x01 || int2octets(x) || bits2octets(h1))
    // V = HMAC_K(V)
    k = hmac(
        &k,
        &[
            &v[..],
            &[0x01][..],
            &int_2_octets::<N>(BigUint::from_bytes_be(e))[..],
            &z[..],
        ]
        .concat(),
    );
    let mut v = hmac(&k, &v[..]);

    let mut t = [0u8; N];

    loop {
        let mut toff = 0;
        while toff < N {
            // V = HMAC_K(V)
            v = hmac(&k, &v[..]);
            let cc = min(v.len(), N - toff);
            t[toff..toff + cc].copy_from_slice(&v[..cc]);
            toff += cc;
        }

        let k_candidate = bits_2_int(&t, q_bi.bits());
        if k_candidate != BigUint::zero() && k_candidate < q_bi {
            return k_candidate;
        }

        // K = HMAC_K(V || 0x00)
        // V = HMAC_K(V)
        k = hmac(&k, &[&v[..], &[0x00][..]].concat());
        v = hmac(&k, &v[..]);
    }
}

fn hmac<const K: usize>(k: &[u8; K], h: &[u8]) -> [u8; K] {
    let mut mac = Hmac::<Sha256>::new_from_slice(k.as_slice())
        .expect("Error initializing HMAC key from slice");

    mac.update(h);

    let result = mac.finalize();

    let k = result
        .into_bytes()
        .as_slice()
        .try_into()
        .expect("Wrong length when converting HMAC result to slice");

    k
}

fn bits_2_int(vb: &[u8], qlen: u64) -> BigUint {
    let mut v = BigUint::from_bytes_be(&vb);
    let vlen = vb.len() * 8;
    if vlen > qlen as usize {
        v >>= vlen - qlen as usize;
    }
    v
}

fn int_2_octets<const N: usize>(n: BigUint) -> [u8; N] {
    let n = n.to_bytes_be();
    let mut buffer = [0u8; N];

    if n.len() < N {
        let diff = N - n.len();
        buffer[diff..].copy_from_slice(&n[..]);
    } else if n.len() > N {
        let diff = n.len() - N;
        buffer[..].copy_from_slice(&n[diff..]);
    } else {
        buffer[..].copy_from_slice(&n[..]);
    }
    buffer
}

fn bits_2_octets<const N: usize>(n: &[u8], q: &[u8]) -> [u8; N] {
    let q = BigUint::from_bytes_be(q);
    let mut n = bits_2_int(&n, q.bits());

    if n >= q {
        n -= q;
    }

    int_2_octets(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::sha256;
    use hex;

    #[test]
    fn test_int_2_octets() {
        let n = BigUint::from(1u32);

        let qlen = n.bits();
        let rolen = (qlen + 7) >> 3;
        let rlen = rolen * 8;

        assert_eq!(qlen, 1);
        assert_eq!(rolen, 1);
        assert_eq!(rlen, 8);

        let result: [u8; 1] = int_2_octets(n.clone());
        assert_eq!(result, [0x01]);

        let result: [u8; 2] = int_2_octets(n.clone());
        assert_eq!(result, [0x00, 0x01]);

        let result: [u8; 3] = int_2_octets(n);
        assert_eq!(result, [0x00, 0x00, 0x01]);

        let n = BigUint::from(1u64 << 33);

        let qlen = n.bits();
        let rolen = (qlen + 7) >> 3;
        let rlen = rolen * 8;

        assert_eq!(qlen, 34);
        assert_eq!(rolen, 5);
        assert_eq!(rlen, 40);

        let result: [u8; 5] = int_2_octets(n.clone());
        assert_eq!(result, [0x02, 0x00, 0x00, 0x00, 0x00]);

        let result: [u8; 6] = int_2_octets(n.clone());
        assert_eq!(result, [0x00, 0x02, 0x00, 0x00, 0x00, 0x00]);

        let result: [u8; 7] = int_2_octets(n.clone());
        assert_eq!(result, [0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_bits_2_octets() {
        let q = hex::decode("04000000000000000000020108a2e0cc0d99f8a5ef").unwrap();
        let z = hex::decode("af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d0268e9891562113d8a62add1bf")
            .unwrap();

        let z_octets: [u8; 21] = bits_2_octets(&z, &q);

        assert_eq!(
            hex::encode(z_octets),
            "01795edf0d54db760f156d0dac04c0322b3a204224"
        );
    }

    #[test]
    fn test_ints_2_octets() {
        let e = hex::decode("9a4d6792295a7f730fc3f2b49cbc0f62e862272f").unwrap();

        let e: [u8; 21] = int_2_octets(BigUint::from_bytes_be(&e));

        assert_eq!(hex::encode(e), "009a4d6792295a7f730fc3f2b49cbc0f62e862272f");
    }

    #[test]
    fn test_bits_2_int() {
        let t = hex::decode("9305a46de7ff8eb107194debd3fd48aa20d5e7656cbe0ea69d2a8d4e7c67314a")
            .unwrap();

        let result = hex::encode(bits_2_int(&t, 163).to_bytes_be());

        assert_eq!(result, "04982d236f3ffc758838ca6f5e9fea455106af3b2b");

        let t = hex::decode("c70c78608a3b5be9289be90ef6e81a9e2c1516d5751d2f75f50033e45f73bdeb")
            .unwrap();

        let result = hex::encode(bits_2_int(&t, 163).to_bytes_be());

        assert_eq!(result, "063863c30451dadf4944df4877b740d4f160a8b6ab");

        let t = hex::decode("475e80e992140567fcc3a50dab90fe84bcd7bb03638e9c4656a06f37f6508a7c")
            .unwrap();

        let result = hex::encode(bits_2_int(&t, 163).to_bytes_be());

        assert_eq!(result, "023af4074c90a02b3fe61d286d5c87f425e6bdd81b");
    }

    #[test]
    fn test_generate_k() {
        let q: Vec<u8> = hex::decode("04000000000000000000020108a2e0cc0d99f8a5ef").unwrap();
        let e = hex::decode("009a4d6792295a7f730fc3f2b49cbc0f62e862272f").unwrap();
        let z = hex::decode("af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d0268e9891562113d8a62add1bf")
            .unwrap();

        let qlen = BigUint::from_bytes_be(&q).bits();
        let rolen = (qlen + 7) >> 3;
        let rlen = rolen * 8;

        assert_eq!(qlen, 163);
        assert_eq!(rolen, 21);
        assert_eq!(rlen, 168);

        let q: [u8; 21] = int_2_octets(BigUint::from_bytes_be(&q));
        let e: [u8; 21] = int_2_octets(BigUint::from_bytes_be(&e));
        let z: [u8; 21] = bits_2_octets(&z, &q);

        let k = generate_k::<21, 32>(&z, &e, &q);

        assert_eq!(
            hex::encode(k.to_bytes_be()),
            "023af4074c90a02b3fe61d286d5c87f425e6bdd81b"
        );
    }

    #[test]
    fn test_vector_dsa_1024_bits() {
        let q = &hex::decode("996f967f6c8e388d9e28d01e205fba957a5698b1").unwrap();
        let e = hex::decode("411602cb19a6ccc34494d79d98ef1e7ed5af25f7").unwrap();
        let z = sha256("sample");

        let qlen = BigUint::from_bytes_be(&q).bits();
        let rolen = (qlen + 7) >> 3;
        let rlen = rolen * 8;

        assert_eq!(qlen, 160);
        assert_eq!(rolen, 20);
        assert_eq!(rlen, 160);

        let q: [u8; 20] = int_2_octets(BigUint::from_bytes_be(&q));
        let e: [u8; 20] = int_2_octets(BigUint::from_bytes_be(&e));
        let z: [u8; 20] = bits_2_octets(&z, &q);
        let k = generate_k::<20, 32>(&z, &e, &q);

        assert_eq!(
            hex::encode(k.to_bytes_be()),
            "519ba0546d0c39202a7d34d7dfa5e760b318bcfb"
        );
    }

    #[test]
    fn test_vector_ecdsa_256_bits() {
        let q = "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551";
        let q = &hex::decode(q).unwrap();

        let e = "c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721";
        let e = hex::decode(e).unwrap();

        let z = sha256("sample");

        let qlen = BigUint::from_bytes_be(&q).bits();
        let rolen = (qlen + 7) >> 3;
        let rlen = rolen * 8;

        assert_eq!(qlen, 256);
        assert_eq!(rolen, 32);
        assert_eq!(rlen, 256);

        let q: [u8; 32] = int_2_octets(BigUint::from_bytes_be(&q));
        let e: [u8; 32] = int_2_octets(BigUint::from_bytes_be(&e));
        let z: [u8; 32] = bits_2_octets(&z, &q);

        let k = generate_k::<32, 32>(&z, &e, &q);

        let k_expected = "a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60";
        assert_eq!(hex::encode(k.to_bytes_be()), k_expected);
    }

    #[test]
    fn test_vector_ecdsa_571_bits() {
        let q = "03ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe661ce18ff55987308059b186823851ec7dd9ca1161de93d5174d66e8382e9bb2fe84e47";
        let q = &hex::decode(q).unwrap();

        let e = "0028a04857f24c1c082df0d909c0e72f453f2e2340ccb071f0e389bca2575da19124198c57174929ad26e348cf63f78d28021ef5a9bf2d5cbeaf6b7ccb6c4da824dd5c82cfb24e11";
        let e = hex::decode(e).unwrap();

        let z = sha256("sample");

        let qlen = BigUint::from_bytes_be(&q).bits();
        let rolen = (qlen + 7) >> 3;
        let rlen = rolen * 8;

        assert_eq!(qlen, 570);
        assert_eq!(rolen, 72);
        assert_eq!(rlen, 576);

        let q: [u8; 72] = int_2_octets(BigUint::from_bytes_be(&q));
        let e: [u8; 72] = int_2_octets(BigUint::from_bytes_be(&e));
        let z: [u8; 72] = bits_2_octets(&z, &q);

        let k = generate_k::<72, 32>(&z, &e, &q);

        let k_exp = "015c2c6b7d1a070274484774e558b69fdfa193bdb7a23f27c2cd24298ce1b22a6cc9b7fb8cabfd6cf7c6b1cf3251e5a1cddd16fbfed28de79935bb2c631b8b8ea9cc4bcc937e669e";
        assert_eq!(hex::encode(k.to_bytes_be()), k_exp);
    }
}
