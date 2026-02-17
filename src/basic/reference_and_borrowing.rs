
#[allow(unused)]
fn calculate_length(s: &String) -> usize {
    s.len()
}

#[allow(unused)]
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[allow(unused)]
fn multiple_mutable_references() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;    
    // println!("r1: {}, r2: {}", r1, r2); // this will work because r1 and r2 are immutable references and they can coexist

    let r3 = &mut s;
    // println!("r3: {}", r3); // this will work because r3 is a mutable reference and it has exclusive access to s
}


// #[allow(unused)]
// fn dangling_reference() -> &String {
//     let s = String::from("Hello");

//     &s
// }
// code above will cause an error because s is dropped at the end of curly brakets
// this the fix version of the code above

#[allow(unused)]
fn no_dangling_reference() -> String {
    let s = String::from("Hello");

    s
}
// return the ownership of s, give that to the caller
// another wat to fix the dangling reference is to return the reference of s but we need to make sure that s is not dropped at the end of the function
// we can do that by using the lifetime annotation
// i will learn about lifetime on the chapter

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s1 = String::from("hello");
        let length = calculate_length(&s1);
        assert_eq!(length, 5);
    }

    #[test]
    fn test_change() {
        let mut hello_string = String::from("hello");
        change(&mut hello_string);
        assert_eq!(hello_string, "hello, world");
    }

    #[test]
    fn test_multiple_mutable_references() {
        multiple_mutable_references();
    }
}