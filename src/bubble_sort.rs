pub fn run(slice: &mut [i32]) {
    let len = slice.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}
