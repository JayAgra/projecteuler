fn main() {
    let mut largest_palindrome: u32 = 0;
    for i in (100..=999).rev() {
        if largest_palindrome < i * 999 {
            for j in (i..=1000).rev() {
                let v = i * j;
                if largest_palindrome < v && is_palindrome(v) {
                    largest_palindrome = v;
                }
            }
        }
    }
    println!("{}", largest_palindrome);
}

fn is_palindrome(num: u32) -> bool {
    let mut reversed: u32 = 0;
    let mut iv = num;
    while iv > 0 {
        reversed = 10 * reversed + iv % 10;
        iv /= 10;
    }
    reversed == num
}