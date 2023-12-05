fn main() {
    let mut multiples: Vec<u32> = Vec::new();
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            multiples.push(i)
        }
    }
    println!("{}", multiples.iter().sum::<u32>());
}
