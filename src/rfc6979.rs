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
            &bits_2_octets::<N>(z, q)[..],
        ]
        .concat(),
    );

    let v = hmac(&k, &v[..]);
    println!("k = {:x?}", k);
    println!("v = {:x?}", v);

    // K = HMAC_K(V || 0x01 || int2octets(x) || bits2octets(h1))
    // V = HMAC_K(V)
    k = hmac(
        &k,
        &[
            &v[..],
            &[0x01][..],
            &int_2_octets::<N>(BigUint::from_bytes_be(e))[..],
            &bits_2_octets::<N>(z, q)[..],
        ]
        .concat(),
    );
    let mut v = hmac(&k, &v[..]);
    println!("k = {:x?}", k);
    println!("v = {:x?}", v);

    let mut t = [0u8; N];
    let mut i = 0;

    while i < 3 {
        // V = HMAC_K(V)
        // let mut v = hmac(&k, &v[..]);

        /*
         * We want qlen bits, but we support only
         * hash functions with an output length
         * multiple of 8;acd hence, we will gather
         * rlen bits, i.e., rolen octets.
         */
        let mut toff = 0;
        while (toff < N) {
            v = hmac(&k, &v[..]);
            let cc = min(v.len(), N - toff);
            t[toff..toff + cc].copy_from_slice(&v[..cc]);
            toff += cc;
        }
        println!("v = {:x?}", v);
        println!("t = {:x?}", t);

        let k_candidate = bits_2_int(&v, 163);
        if k_candidate != BigUint::zero() && k_candidate < q_bi {
            return k_candidate;
        }
        println!("k_bi = {:x?}", hex::encode(k_candidate.to_bytes_be()));

        // K = HMAC_K(V || 0x00)
        // V = HMAC_K(V)
        k = hmac(&k, &[&v[..], &[0x00][..]].concat());
        v = hmac(&k, &v[..]);
        println!("----");
        println!("k = {:x?}", k);
        println!("v = {:x?}", v);
        i += 1;
    }
    BigUint::from(1u32)
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
    if N >= n.len() {
        let diff = N - n.len();
        buffer[diff..].copy_from_slice(&n);
    } else {
        buffer[..].copy_from_slice(&n[..N]);
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
    use hex;

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
        let q = hex::decode("04000000000000000000020108a2e0cc0d99f8a5ef").unwrap();
        let e = hex::decode("009a4d6792295a7f730fc3f2b49cbc0f62e862272f").unwrap();
        let z = hex::decode("af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d0268e9891562113d8a62add1bf")
            .unwrap();

        let qlen = BigUint::from_bytes_be(&q).bits();
        let rolen = (qlen + 7) >> 3;
        let rlen = rolen * 8;
        println!("qlen = {}", qlen);
        println!("rolen = {}", rolen);
        println!("rlen = {}", rlen);

        let q: [u8; 21] = int_2_octets(BigUint::from_bytes_be(&q));
        let e: [u8; 21] = int_2_octets(BigUint::from_bytes_be(&e));
        println!("int_2_octets (q) = {:x?}", q);
        println!("int_2_octets (e) = {:x?}", e);

        let z: [u8; 21] = bits_2_octets(&z, &q);
        let k = generate_k::<21, 32>(&z, &e, &q);
    }
}