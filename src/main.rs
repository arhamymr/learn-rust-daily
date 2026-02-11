mod basic;
mod cryptography;
mod networking;

#[allow(unused_imports)]
fn main() {
    println!("Hello, world!");
    let result = basic::add_with_10(5);
    println!("Result: {}", result);

    let value = "example";
    let hash = cryptography::calculate_hash(&value);
    println!("Hash of '{}': {}", value, hash);

    let hash_sha256 = cryptography::sha256_hash(b"example");
    println!("SHA256 Hash: {}", hash_sha256);

    networking::http();
    println!("HTTP module executed successfully.");
}
