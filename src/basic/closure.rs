fn add_with_10(x: u32) -> u32 {
    let result = |num| num + x;
    return result(10)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_with_10() {
        let res = add_with_10(5);
        assert_eq!(res, 15);
    }
}

