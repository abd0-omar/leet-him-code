pub fn check_inclusion(s1: String, s2: String) -> bool {
    // first thoughts that comes to mind,
    // check the frequency
    // and sorting
    // we will go with the frequency one without sorting for now
    //
    // "ab"
    // "eidbaooo"
    //
    // for every string of size 2 in the bigger string check if it has the frequency of the lower
    // string

    // let mut freq = vec![0; 26];
    // // assume that we know that s1 is the lower string
    // for letter in s1.as_bytes() {
    //     let letter_idx = (letter - b'a') as usize;
    //     freq[letter_idx] += 1;
    // }

    if s1.len() > s2.len() {
        return false;
    }

    // we want to check if s1 in s2 not the lower string
    // so this is redundant
    let (lower, bigger) = if s1.len() <= s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };

    'outer: for i in 0..bigger.len() {
        let mut freq = vec![0; 26];
        for letter in lower.as_bytes() {
            let letter_idx = (letter - b'a') as usize;
            freq[letter_idx] += 1;
        }
        dbg!(i);
        for j in i..lower.len() + i {
            if j == bigger.len() {
                continue 'outer;
            }
            let letter = bigger.as_bytes()[j];
            // dbg!(&freq);
            dbg!(char::from_u32(letter as u32));
            let letter_idx = (letter - b'a') as usize;
            if freq[letter_idx] > 0 {
                freq[letter_idx] -= 1;
            } else {
                dbg!(j);
                continue 'outer;
            }
        }
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        let output = true;
        let result = check_inclusion(s1, s2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        let output = false;
        let result = check_inclusion(s1, s2);
        assert_eq!(result, output);
    }
}
