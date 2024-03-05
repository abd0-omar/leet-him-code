fn main() {
    println!("Hello, world!");
    // Example 1:

    let s = "lEetcOde".to_string();
    // Output: "lEOtcede".to_string();
    // Explanation: 'E', 'O', and 'e' are the vowels in s; 'l', 't', 'c', and 'd' are all consonants. The vowels are sorted according to their ASCII values, and the consonants remain in the same places.
    println!("{}", sort_vowels(s));
}

pub fn sort_vowels(mut s: String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut sorted_indices: Vec<_> = s
        // 0, 'a'
        .char_indices()
        .filter(|(_, x)| vowels.contains(&x.to_lowercase().next().unwrap()))
        .map(|(x, _)| x)
        .collect();
    println!("sorted_indices={:?}", sorted_indices);
    sorted_indices.sort_by(|&a, &b| s.as_bytes()[a].cmp(&(s.as_bytes()[b] as u8)));
    println!("sorted_indices={:?}", sorted_indices);
    let mut sorted_indices = sorted_indices.iter();

    let mut new = s.clone();

    for (i, c) in s.as_bytes().iter().enumerate() {
        if vowels.contains(&(*c as char).to_lowercase().next().unwrap()) {
            let f = sorted_indices.next().unwrap();
            println!("f={:?}", f);
            new.replace_range(i..i + 1, &s.chars().nth(*f).unwrap().to_string());
        }
    }
    println!("s={:?}", new);
    new
    // let s = s.as_bytes();
    // let mut sorted_indices: Vec<_> = (0..s.len()).collect();
    // sorted_indices.sort_by(|&a, &b| s[a].cmp(&(b as u8)));
    // println!("sorted_indices={:?}", sorted_indices);
    // for (i, &c) in s.iter().enumerate() {
    //     if vowels.contains(&(c as char)) {
    //         for j in 0..sorted_indices.len() {
    //             if s[j]
    //         }
    //     }
    // }
}
