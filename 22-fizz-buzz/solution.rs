pub fn fizz_buzz(num: u32) -> String {
    // TODO: Implement the FizzBuzz challenge
    if num % 5 == 0 && num % 3 == 0 {
        String::from("FizzBuzz")
    } else if num % 5 == 0 {
        String::from("Buzz")
    } else if num % 3 == 0 {
        String::from("Fizz")
    } else {
        num.to_string()
    }
}

