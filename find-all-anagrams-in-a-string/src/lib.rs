pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    // TLE
    if p.len() > s.len() {
        return Vec::new();
    }

    let s = s.into_bytes();
    let mut p = p.into_bytes();

    // let mut hm = std::collections::HashMap::new();
    // for &letter in p.iter() {
    //     hm.entry(letter)
    //         .and_modify(|counter| *counter += 1)
    //         .or_insert(1);
    // }

    // let mut hs = std::collections::HashSet::new();
    p.sort_unstable();
    // hs.insert(&p);

    // let mut result = Vec::new();
    // for i in 0..s.len() - p.len() + 1 {
    //     let mut hm2 = std::collections::HashMap::new();
    //     for j in i..i + p.len() {
    //         hm2.entry(s[j])
    //             .and_modify(|counter| *counter += 1)
    //             .or_insert(1);
    //     }
    //     if hm == hm2 {
    //         result.push(i as i32);
    //     }
    // }

    let mut result = Vec::new();
    for i in 0..s.len() - p.len() + 1 {
        // let mut hs2 = std::collections::HashSet::new();
        // dbg!(i + p.len());
        // dbg!(i);
        let mut f = vec![0; p.len()];
        f.clone_from_slice(&s[i..i + p.len()]);
        // dbg!(String::from_utf8(f.clone()).unwrap());
        f.sort_unstable();
        // dbg!(String::from_utf8(f.clone()).unwrap());
        // hs2.insert(&f);
        if p == f {
            result.push(i as i32);
        }
    }
    // idk why I used a hashset, it does nothing
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        /*
        p ->  a|1, b|1, c|1

        s:
        c -> 1
        b -> 1
        a -> 1
        so we could traverse every 3 len substring and check it's freq
        */
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        let output = vec![0, 6];
        // Explanation:
        // The substring with start index = 0 is "cba", which is an anagram of "abc".
        // The substring with start index = 6 is "bac", which is an anagram of "abc".
        let result = find_anagrams(s, p);
        assert_eq!(result, output);
    }
}
