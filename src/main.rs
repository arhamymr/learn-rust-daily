mod basic;
mod cryptography;
mod networking;


#[allow(unused_imports)]
fn main() {
    println!("Hello, world!");
    let result = basic::closure::add_with_10(5);
    println!("Result: {}", result);

    let value = "example";
    let hash = cryptography::calculate_hash(&value);
    println!("Hash of '{}': {}", value, hash);

    let hash_sha256 = cryptography::sha256_hash(b"example");
    println!("SHA256 Hash: {}", hash_sha256);

    networking::http();
    println!("HTTP module executed successfully.");

    basic::primitives::print_example_literals_and_operators();
    basic::primitives::print_example_tuples_and_arrays();
    basic::primitives::print_example_array_slice();
    let array_slice = basic::primitives::example_array_slice();
    println!("Returned array slice: {:?}", array_slice);


    basic::control_flow::nested_loops();
    
}
