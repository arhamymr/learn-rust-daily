// Hash best use case is for : 
// 1. Datay: Hases can be used to verify the integrity of data. By comparing the hash of the original data with the hash of the received data, you can determine if the data has been altered or corrupted during transmission.
// 2. Blockchain
// 3. Digital Signatures 
// 4. HMAC (Hash-based Message Authentication Code)
// 5. Hash Data Umum  

use std::hash::{DefaultHasher, Hash, Hasher};
use sha2::{Sha256, Digest};

pub fn calculate_hash<T:Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

pub fn sha256_hash(data: &[u8]) -> String {
    let result = Sha256::digest(data);
    format!("{:x}", result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash)]
    struct HashedStructExample {
        a: i32,
        b: String,
    }

    #[test]
    fn test_calculate_hash() {
        let value1 = "hello";
        let value2 = "world";

        let struct_hash = HashedStructExample {
            a: 42, 
            b: String::from("example"),
        };

        let hash1 = calculate_hash(&value1);
        let hash2 = calculate_hash(&value2);
        let hash3 = calculate_hash(&struct_hash);
        
        println!("Hash of '{}': {}", value1, hash1);
        println!("Hash of '{}': {}", value2, hash2);
        println!("Hash of struct: {}", hash3);

        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_ne!(hash2, hash3);
    }

    #[test]
    fn test_sha256_hash() {
        let data = "example".as_bytes();
        let hash = sha256_hash(data);
        assert_eq!(hash, "50d858e0985ecc7f60418aaf0cc5ab587f42c2570a884095a9e8ccacd0f6545c");
    }
}