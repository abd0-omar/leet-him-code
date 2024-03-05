fn main() {
    println!("Hello, world!");
    let s = "aaa".to_string();
    // Output: 6
    println!("count_substrings={:?}", count_substrings(s));
    let s = "racecar".to_string();
    // Output: 3
    println!("count_substrings={:?}", count_substrings(s));
}

pub fn count_substrings(s: String) -> i32 {
    let n = s.len();
    let mut count = 0;
    let mut memory = vec![vec![None; n]; n];

    for i in 0..n {
        for j in i..n {
            if is_palindrome(&s.as_bytes(), i as i32, j as i32, &mut memory) {
                dbg!(&s[i..=j]);
                count += 1;
            }
        }
    }

    count
}

fn is_palindrome(substring: &[u8], i: i32, j: i32, memory: &mut Vec<Vec<Option<bool>>>) -> bool {
    if i > j {
        return true;
    }

    if let Some(ret) = memory[i as usize][j as usize] {
        println!("hello");
        return ret;
    }

    if substring[i as usize] != substring[j as usize] {
        return false;
    }

    let result = is_palindrome(substring, i + 1, j - 1, memory);

    memory[i as usize][j as usize];

    result
}
