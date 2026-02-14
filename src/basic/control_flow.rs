fn control_flow_example(x: i32) -> String {
    let number = x;

    if number < 5 {
        format!("The value of x is less than 5: {}", number)
    } else {
        format!("The value of x is: {}", number)
    }
}

fn repetion_with_loop() -> String {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    return format!("Result: {}", result);
}

pub fn nested_loops() {
    let mut global_counter = 1;
    let mut count = 1;

    loop {
        if global_counter >= 100 {
            break;
        }

        println!("{} outer loop iteration: {}", global_counter, count);
        global_counter += 1;

        let mut inner_count = 1;

       

        'loop2: loop {
            println!("{} Inner Loop iteration: {}", global_counter, count);
            global_counter += 1;

            inner_count += 1;
            if inner_count == 10 {
                count += 1;
                break 'loop2; // Breaks out of the inner loop
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_flow_example() {
        let result = control_flow_example(7);
        assert_eq!(result, "The value of x is: 7");

        let less_than_five_result = control_flow_example(3);
        assert_eq!(less_than_five_result, "The value of x is less than 5: 3");

        let repetion_result = repetion_with_loop();
        assert_eq!(repetion_result, "Result: 10");
    }
}
