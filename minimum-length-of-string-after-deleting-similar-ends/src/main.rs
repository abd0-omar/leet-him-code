fn main() {
    println!("Hello, world!");
    let s = "aabccabba".to_string();
    println!("{}", minimum_length(s));
    let s = "ca".to_string();
    // Output: 2
    println!("{}", minimum_length(s));
    let s = "cabaabac".to_string();
    // Output: 0
    println!("{}", minimum_length(s));
    let s = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbacccabbabccaccbacaaccacacccaccbbbacaabbccbbcbcbcacacccccccbcbbabccaacaabacbbaccccbabbcbccccaccacaccbcbbcbcccabaaaabbbbbbbbbbbbbbb".to_string();
    println!("{}", minimum_length(s));
}

pub fn minimum_length(s: String) -> i32 {
    let mut st = 0;
    let mut end = s.len() - 1;
    let s = s.as_bytes();

    while st <= end {
        println!("st={:?}", st);
        println!("end={:?}", end);
        if s[st] == s[end] {
            while s.get(st + 1).is_some() {
                if s[st + 1] == s[st] {
                    st += 1;
                } else {
                    st += 1;
                    break;
                }
            }

            while s.get(end - 1).is_some() {
                if s[end - 1] == s[end] {
                    end -= 1;
                } else {
                    end -= 1;
                    break;
                }
            }
        } else {
            break;
        }
    }

    let rezult = end as i32 - st as i32;
    if rezult < 0 {
        0
    } else {
        rezult + 1
    }
}
