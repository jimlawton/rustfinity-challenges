// Define a struct named `Logger`
pub struct Logger;

// Implement an associated function `log_message`
// That accepts a `&str` and prints the output.
impl Logger {
    pub fn log_message(msg: &str) {
        println!("{}", msg);
    }
}

// Example usage:
pub fn main() {
    Logger::log_message("Hello, World!");
}

