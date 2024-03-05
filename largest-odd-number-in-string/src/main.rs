fn main() {
    println!("Hello, world!");
}

pub fn largest_odd_number(num: String) -> String {
    for (i, n_char) in num.char_indices().rev() {
        let n_num = n_char.to_digit(10).unwrap();
        if n_num % 2 == 1 {
            return num[..=i].to_string();
        }
    }

    String::new()
}
