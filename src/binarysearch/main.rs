mod binarysearch;
use binarysearch::search;

fn main() {
    let mut nums: [i32; 15] = [6, 33, 43, 51, 53, 13, 14, 93, 95, 96, 97, 25, 64, 72, 84];
    nums.sort();
    println!("{:?}", nums);
    println!("search(&nums, 33)");
    println!("{}", search(&nums, 33));
}
