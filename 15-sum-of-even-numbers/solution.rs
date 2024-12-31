pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    // Your code here...
    if start > end {
        return 0;
    } else {
        let range = std::ops::Range { start, end: end + 1 };
        range.filter(|a| if a % 2 == 0 {true} else {false}).sum()
    }
}

