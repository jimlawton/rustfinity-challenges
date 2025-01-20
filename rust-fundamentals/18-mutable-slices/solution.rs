pub fn transform_even_odd(slice: &mut [i32]) {
    // Your code here: iterate over the mutable slice and modify its elements.
    for value in slice.iter_mut() {
        if *value % 2 == 0 {
            *value *= 2;
        } else {
            *value -= 1;
        }
    }
}

