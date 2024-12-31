pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    // TODO: Implement the function here
    let range = if start > end {
        end..start+1
    } else {
        start..end+1
    };
    for i in range {
        let num_str: String = format!("{i}");
        let rev_str: String = num_str.chars().rev().collect();
        if num_str == rev_str {
            return Some(i);
        }
    }
    None
}

