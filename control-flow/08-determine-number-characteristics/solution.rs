pub fn describe_number(n: i32) -> String {
    // TODO: Implement the function here
    if n > 0 {
        if n % 2 == 0 {
            String::from("Positive even")
        } else {
            String::from("Positive odd")
        }
    } else if n < 0 {
        if n % 2 == 0 {
            String::from("Negative even")
        } else {
            String::from("Negative odd")
        }
    } else {
        String::from("Zero")
    }
}

