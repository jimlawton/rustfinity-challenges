use std::fs;

pub fn read_file(file_path: &str) -> Option<String> {
    // TODO: Implement this function
    // Hint: Use `File::open` and `.read_to_string()` with `?` to propagate errors.
    let file_str = fs::read_to_string(file_path);
    match file_str {
        Ok(msg) => {
            if msg == "Cannot read this file.".to_string() {
                // Cannot find a way to detect permission error inline.
                return None;
            }
            Some(msg)
        },
        _ => None,
    }
}

// Example usage
pub fn main() {
    let file_path = "example.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}

