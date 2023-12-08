fn main() {
    println!("{}", get_primes(10001).last().unwrap());
}

fn get_primes(number: usize) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    let mut i: u64 = 2;
    while primes.len() < number {
        let mut is_prime: bool = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
        i += 1;
    }
    primes
}