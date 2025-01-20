pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here
    for &index in indices {
        if index < slice.len() {
            if let Some(value_ref) = slice.get_mut(index) {
                *value_ref = value;
            }
        }
    }
}

