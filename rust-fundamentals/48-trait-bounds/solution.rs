use std::cmp::PartialOrd;
use std::fmt::Display;

pub fn compare_and_display<T: Display + PartialOrd>(value1: T, value2: T) -> T {
     println!("Value1: {value1}");
     println!("Value2: {value2}");
     if value1 > value2 {
          value1
     } else {
          value2
     }
}

// Example usage
pub fn main() {
     let greater = compare_and_display::<u32>(10, 20);
     println!("Greater value: {}", greater);

     let greater = compare_and_display::<&str>("Apple", "Orange");
     println!("Greater value: {}", greater);
}

