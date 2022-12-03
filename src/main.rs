mod ch3;
use ch3::Solution;

fn main() {
    let s = std::env::args().nth(1).expect("provide string");
    println!("result: {}", Solution::length_of_longest_substring(s));
}
