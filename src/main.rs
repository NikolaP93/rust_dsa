mod binary_search;

fn main() {
    let ordered_list = [1, 2, 4, 7, 141, 642, 777, 1246, 10991];

    binary_search::run(&ordered_list, 7);
}
