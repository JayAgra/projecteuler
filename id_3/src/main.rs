fn main() {
    let input_number: u128 = 600851475143;
    let mut prime_factors: Vec<u128> = Vec::new();
    let mut iterator: Box<dyn Iterator<Item = _>> = Box::new(2..=(input_number as f32).sqrt().floor() as _);
    while let Some(prime_number) = iterator.next() {
        iterator = Box::new(iterator.filter(move |&x| x % prime_number != 0));
        if input_number % (prime_number as u128) == 0 {
            prime_factors.push(prime_number);
        }
    }
    println!("{}", prime_factors.iter().max().unwrap());
}