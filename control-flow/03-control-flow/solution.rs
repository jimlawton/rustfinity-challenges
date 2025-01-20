pub fn countdown(n: u32) -> Vec<u32> {
    // TODO: Implement the countdown using a while loop
    let mut v = Vec::new();
    if n == 0 {
        v.push(0)
    } else {
        for i in (std::ops::Range {start: 0, end: n + 1}) {
            v.push(n - i);
        }
    }
    v
}

