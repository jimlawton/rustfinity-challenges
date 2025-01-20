use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    // TODO: Implement logic here to return the median of the list
    numbers.sort();
    if numbers.len() % 2 == 0 {
        (numbers[numbers.len() / 2] as f32 + numbers[numbers.len() / 2 - 1] as f32) / 2.0
    } else {
        numbers[numbers.len() / 2] as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    // TODO: Implement logic here to return the mode of the list
    let mut results = Vec::new();
    let mut map = HashMap::<i32, i32>::new();
    for &i in numbers {
        map.entry(i).and_modify(|v| *v += 1).or_insert(1);
    }
    let &max_frequency = map.values().clone().max().unwrap();
    for (&k, &v) in map.iter() {
        if v == max_frequency {
            results.push(k);
        }
    };
    results.sort();
    results
}

