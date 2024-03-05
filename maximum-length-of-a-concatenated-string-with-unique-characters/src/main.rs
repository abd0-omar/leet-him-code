use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    // Example 2:

    let arr = vec![
        "cha".to_string(),
        "r".to_string(),
        "act".to_string(),
        "ers".to_string(),
    ];
    // Output: 6
    // Explanation: Possible longest valid concatenations are "chaers" ("cha" + "ers") and "acters" ("act" + "ers").
    println!("{}", max_length(arr));
    let arr = vec!["un".to_string(), "iq".to_string(), "ue".to_string()];
    println!("{}", max_length(arr));
    let arr = vec!["abcdefghijklmnopqrstuvwxyz".to_string()];
    println!("{}", max_length(arr));
    let arr = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
        "f".to_string(),
        "g".to_string(),
        "h".to_string(),
        "i".to_string(),
        "j".to_string(),
        "k".to_string(),
        "l".to_string(),
        "m".to_string(),
        "n".to_string(),
        "o".to_string(),
        "p".to_string(),
    ];
    println!("{}", max_length(arr));
    let arr = vec!["aa".to_string(), "bb".to_string()];
    println!("{}", max_length(arr));
    let arr = vec!["jnfbyktlrqumowxd".to_string(), "mvhgcpxnjzrdei".to_string()];
    println!("{}", max_length(arr));
    let arr = vec![
        "a".to_string(),
        "abc".to_string(),
        "d".to_string(),
        "de".to_string(),
        "def".to_string(),
    ];
    println!("{}", max_length(arr));
}

pub fn max_length(mut arr: Vec<String>) -> i32 {
    arr.sort_unstable_by(|a, b| b.cmp(a));
    let n = arr.len();
    // if n == 1 {
    //     return arr[0].len() as _;
    // }
    let mut max_length = 0;
    let mut inner_repeat = false;
    'outer: for i in 0..n {
        let mut maxez = 0;
        let mut hs = HashSet::new();
        for c in arr[i].chars() {
            if !hs.insert(c) {
                inner_repeat = true;
                println!("c={:?}", c);
                continue 'outer;
            }
        }

        'inner: for j in i + 1..n {
            let mut appeared = false;

            for c in arr[j].chars() {
                if hs.contains(&c) {
                    appeared = true;
                    break;
                }
            }

            if !appeared {
                for c in arr[j].chars() {
                    if !hs.insert(c) {
                        continue 'inner;
                    }
                }
                if maxez == 0 {
                    maxez = maxez.max(arr[i].len() + arr[j].len());
                } else {
                    maxez = maxez + arr[j].len();
                }
                // println!("hs={:?}", hs);
            }
        }

        if !inner_repeat {
            max_length = max_length.max(arr[i].len());
        }

        max_length = max_length.max(maxez);
    }

    max_length as _
}
