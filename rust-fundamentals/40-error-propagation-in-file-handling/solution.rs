use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    // TODO: Implement this function
    // Hint: Use `File::open`, `BufReader::new`, and `.lines()` to process the file.
    // Use `?` to propagate errors and `io::Error::new` for custom errors.
    let f = File::open(file_path)?;
    let reader = io::BufReader::new(f);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut sum: i32 = 0;
    for line in lines {
        if let Ok(val) = line.parse::<i32>() {
            sum += val;
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid number"));
        }
    }
    Ok(sum)
}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

