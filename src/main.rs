mod binary_search;
mod bubble_sort;

fn main() {
    // let ordered_list = [1, 2, 4, 7, 141, 642, 777, 1246, 10991];
    let mut unordered_list = [7, 6, 44, 1, 0, -24, 56, 11];

    // binary_search::run(&ordered_list, 7);
    bubble_sort::run(&mut unordered_list);
    println!("{:?}", unordered_list);
}
