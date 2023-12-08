fn main() {
    println!("{}", square_of_sum(100) - sum_of_squares(100));
}

fn sum_of_squares(nums: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..=nums {
        sum += i.pow(2);
    }
    sum
}

fn square_of_sum(nums: i64) -> i64 {
    ((nums * (nums + 1)) / 2).pow(2)
}