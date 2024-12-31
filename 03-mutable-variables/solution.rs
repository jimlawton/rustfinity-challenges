pub fn mutating_variables() -> &'static str {
    // 1. Declare a mutable variable `text` with value "hello"
    let mut text = "Hello";
    // 2. Reassign the value of `text` to a new value
    text = "World";
    // 3. Return the value of `text`
    text
}

