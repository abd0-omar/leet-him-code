fn main() {
    println!("Hello, world!");
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();
    println!("{}", str_str(haystack, needle));
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack;
    let needle = needle;

    for i in 0..=(haystack.len() - needle.len()) {
        let sub = &haystack[i..(i + needle.len())];

        if sub == needle {
            return i as i32;
        }
    }

    -1 // Return -1 if the needle is not found
}
