pub fn parse_percentage(input: &str) -> Result<u8, String> {
    // TODO: Implement the function here
    match input.parse::<i32>() {
        Ok(n) => {
            if n < 0 || n > 100 {
                Err(String::from("Percentage out of range"))
            } else {
                Ok(n as u8)
            }
        },
        Err(_) => Err(String::from("Invalid input")),
    }
}

