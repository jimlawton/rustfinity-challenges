pub fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.chars().count() > string2.chars().count() {
        string1
    } else if string1.chars().count() < string2.chars().count() {
        string2
    } else {
        string1
    }
}

// Example usage
pub fn main() {
    let s1 = "short";
    let s2 = "longer string";

    let result = longest(s1, s2);
    println!("The longest string is: {}", result);
    assert_eq!(result, "longer string");

    let s3 = "equal";
    let s4 = "equal";
    let result = longest(s3, s4);
    println!("The longest string is: {}", result);
    assert_eq!(result, "equal");
}

