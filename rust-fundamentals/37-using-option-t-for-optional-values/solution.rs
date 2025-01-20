pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    let evens = numbers.iter().filter(|&&val| val % 2 == 0).collect::<Vec<&i32>>();
    if evens.len() == 0 { None } else { Some(*evens[0]) }
}

// Example usage
pub fn main() {
    let nums1 = vec![1, 3, 5, 8];
    let nums2 = vec![1, 3, 5];

    println!("{:?}", find_first_even(&nums1)); // Output: Some(8)
    println!("{:?}", find_first_even(&nums2)); // Output: None
}

