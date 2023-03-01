#![allow(dead_code)]

use hex;
use sha256::digest;

pub fn sha256_double(z: &str) -> Vec<u8> {
    let z = hex::decode(digest(z)).unwrap();
    let z: &[u8] = &z;
    hex::decode(digest(z)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_hash() {
        let z = sha256_double("my message");

        assert_eq!(
            hex::encode(z),
            "0231c6f3d980a6b0fb7152f85cee7eb52bf92433d9919b9c5218cb08e79cce78"
        );
    }
}
