fn main() {
    println!("{}", get_value());
}

fn greatest_common_denominator(value_a: u64, value_b: u64) -> u64 {
    let (mut a, mut b): (u64, u64) = (value_a, value_b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn get_value() -> u64 {
    let mut sum: u64 = 1;
    for i in 1..21 {
        sum = (sum * i) / greatest_common_denominator(i, sum);
    }
    sum
}