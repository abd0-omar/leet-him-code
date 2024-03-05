fn main() {
    println!("Hello, world!");
}

pub fn halves_are_alike(s: String) -> bool {
    let mut count = 0;
    let half = s.len() / 2;
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    for c in s.chars().take(half) {
        if vowels.contains(&c) {
            count += 1;
        }
    }

    for c in s.chars().skip(half) {
        if vowels.contains(&c) {
            count -= 1;
        }
    }

    count == 0
}
