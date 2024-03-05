fn main() {
    println!("Hello, world!");
    // Example 3:

    let word1 = "cabbba".to_string();
    let word2 = "abbccc".to_string();
    println!("{}", close_strings(word1, word2));
    let word1 = "abc".to_string();
    let word2 = "bca".to_string();
    println!("{}", close_strings(word1, word2));
    let word1 = "a".to_string();
    let word2 = "aa".to_string();
    println!("{}", close_strings(word1, word2));
    let word1 = "uau".to_string();
    let word2 = "ssx".to_string();
    println!("{}", close_strings(word1, word2));
    // Output: true
    // Explanation: You can attain word2 from word1 in 3 operations.
    // Apply Operation 1: "cabbba" -> "caabbb"
    // Apply Operation 2: "caabbb" -> "baaccc"
    // Apply Operation 2: "baaccc" -> "abbccc"
}

pub fn close_strings(word1: String, word2: String) -> bool {
    let mut hm = std::collections::HashMap::new();
    // at index 2 (occured 2 times) there is 'a'

    for w in word1.as_bytes() {
        *hm.entry(w).or_insert(0) += 1;
    }
    println!("hm={:?}", hm);

    let mut freq_numbers = hm.values().map(|&x| x).collect::<Vec<i32>>();
    println!("inv_freq={:?}", freq_numbers);

    let mut hm2 = std::collections::HashMap::new();
    for w in word2.as_bytes() {
        *hm2.entry(w).or_insert(0) += 1;
    }

    for (key, val) in hm2.iter() {
        if !hm.contains_key(key) {
            return false;
        }
        println!("val={:?}", val);
        let mut flag = false;
        for e in freq_numbers.iter_mut() {
            println!("e={:?}", e);
            if e == val {
                *e = 0;
                flag = true;
                println!("how? e={:?}", e);
                break;
            }
        }
        if !flag {
            return false;
        }
    }

    return true;
}
