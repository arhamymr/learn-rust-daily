
#[allow(unused)]
fn ownsership_example() -> String {
    let s = String::from("hello");


    let x = 5;
    let y = x;
    println!("x: {} , y: {}", x, y );


    let s1 = String::from("hello");
    let s2 = s1;

    return s;
}

#[allow(unused)]
fn ownership_and_functions() {


    let s = String::from("Hello, ownership!");
    take_ownership(s); // this move onweship of s to the take ownership and s is no longer valid

    // println!("s: {}", s); // this will cause an error because s is no longer valid

    let x = 10;

    make_copy(x); // this will copy the value of x and x is still valid after this function

    println!("x: {}", x); // this will work because x is still valid


    let s1 = String::from("hello");
    let s2 = give_ownership(s1); // after this function calls s1 is no longer valid because the onweship of s1 is moved to the function and the function return (move) the ownership of s1 to s2;

    println!("s2: {}", s2); // this will work because s2 is valid and it has the ownership of the string

    // println!("s1: {}", s1); // this will cause an error because s1 is no longer valid
}




fn take_ownership(some_string: String) {
    println!("Some string: {}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("Some integer: {}", some_integer);
}


fn give_ownership(some_string: String) -> String {
    some_string
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_example() {
        let result = ownsership_example();
        assert_eq!(result, "hello");
    }
}