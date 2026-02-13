// Scalar types 

// Signed integer : i8, i16, i32, i64, i128, isize
// Unsigned integer : u8, u16, u32, u64, u128, usize
// Floating point : f32, f64
// Boolean type : bool
// Character type : char

pub fn print_example_literals_and_operators() {
    let int_literal = 42; // i32 by default
    let float_literal = 3.14; // f64 by default
    let bool_literal = true;
    let char_literal = 'R';

    println!("Integer literal: {}", int_literal);
    println!("Float literal: {}", float_literal);
    println!("Boolean literal: {}", bool_literal);
    println!("Character literal: {}", char_literal);

    // Arithmetic operators
    let sum = int_literal + 10;
    let difference = int_literal - 10;
    let product = int_literal * 2;
    let quotient = int_literal / 2;
    let remainder = int_literal % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}

pub fn print_example_tuples_and_arrays() {
    let tuple_example = (42, 3.14, 'R');
    let array_example = [1, 2, 3, 4, 5];

    println!("Tuple example: {:?}", tuple_example);
    println!("Array example: {:?}", array_example);

    // Accessing tuple elements
    println!("First element of tuple: {}", tuple_example.0);
    println!("Second element of tuple: {}", tuple_example.1);
    println!("Third element of tuple: {}", tuple_example.2);

    // Accessing array elements
    println!("First element of array: {}", array_example[0]);
    println!("Second element of array: {}", array_example[1]);
    println!("Third element of array: {}", array_example[2]);
}

pub fn print_example_array_slice() {
    let array_example = [1, 2, 3, 4, 5];
    let slice_example = &array_example[1..4]; // This creates a slice of the array

    println!("Array example: {:?}", array_example);
    println!("Slice example: {:?}", slice_example);
}

pub fn example_array_slice() -> Vec<i32> {
    let array_example = [1, 2, 3, 4, 5];
    let slice_example = &array_example[1..4]; // This creates a slice of the array

    slice_example.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literals_and_operators() {
        print_example_literals_and_operators();
    }

    #[test]
    fn test_tuples_and_arrays() {
        print_example_tuples_and_arrays();
    }

    #[test]
    fn test_array_slice() {
        print_example_array_slice();
    }

    #[test]
    fn test_example_array_slice() {
        let array_slice = example_array_slice();
        assert_eq!(array_slice, [2, 3, 4]);
    }

}