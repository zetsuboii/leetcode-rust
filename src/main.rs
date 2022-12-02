mod ch1;
use ch1::Ch1;

fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];

    println!("{:?}", Ch1::two_sum(vec1, 6));
}
