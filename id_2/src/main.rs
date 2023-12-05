fn main() {
    println!("{:?}", get_fibonacci(33).iter().filter(|d| *d % 2 == 0).sum::<u32>());
}

fn get_fibonacci(n: u32) -> Vec<u32> {
    let mut seq: Vec<u32> = vec![1, 1];
    for i in 2..n {
        seq.push(seq[i as usize - 2] + seq[i as usize - 1]);
    };
    seq
}