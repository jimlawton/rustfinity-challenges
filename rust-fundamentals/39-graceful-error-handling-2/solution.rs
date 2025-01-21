use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

impl Error for ParsePercentageError {
    fn description(&self) -> &str {
        match self {
            ParsePercentageError::InvalidInput => "Invalid input",
            ParsePercentageError::OutOfRange => "Out of range",
        }
    }
}

impl std::fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    match input.parse::<i32>() {
        Ok(n) => {
            if n < 0 || n > 100 {
                Err(ParsePercentageError::OutOfRange)
            } else {
                Ok(n as u8)
            }
        },
        Err(_) => Err(ParsePercentageError::InvalidInput),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}

