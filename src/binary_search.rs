pub fn run(slice: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi = slice.len() - 1;

    while lo <= hi {
        let m = lo + (hi - lo) / 2;
        let value = slice[m];
        println!("{value}");
        if value == target {
            return Some(m);
        } else if value < target {
            lo = m + 1;
        } else {
            hi = m - 1;
        }
    }

    None
}
