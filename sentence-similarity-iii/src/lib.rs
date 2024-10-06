pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    /*
    tom jerry
    tom loves jerry


    st  end
    tom jerry
    st        end
    tom loves jerry


    st      end
    tom and jerry
    st        end
    tom loves jerry

        st  end
    tom and jerry
        st    end
    tom loves jerry
    if string ended before the other return false

    st        end
    tom with jerry
    st                  end
    tom is in love with jerry

        st     end
    tom with jerry
        st               end
    tom is in love with jerry

        st     end
    tom with jerry
        st           end
    tom is in love with jerry

        st     end
    tom with jerry
        st           end
    tom is in love with jerry

        st end
    tom with jerry
        st      end
    tom is in love with jerry
    something like that

    if in the smaller string the end passed the st then return true
    */

    let sentence1: Vec<_> = sentence1.split_whitespace().collect();
    let sentence2: Vec<_> = sentence2.split_whitespace().collect();

    let (smol, big) = if sentence1.len() <= sentence2.len() {
        (sentence1, sentence2)
    } else {
        (sentence2, sentence1)
    };

    let mut st_smol = 0;
    let mut end_smol = smol.len() as i32 - 1;

    let mut end_big = big.len() as i32 - 1;

    while st_smol < smol.len() && smol[st_smol] == big[st_smol] {
        st_smol += 1;
    }

    while end_smol >= 0 && smol[end_smol as usize] == big[end_big as usize] {
        end_smol -= 1;
        end_big -= 1;
    }

    end_smol < st_smol as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let sentence1 = "My name is Haley".to_string();
        let sentence2 = "My Haley".to_string();
        let output = true;
        let result = are_sentences_similar(sentence1, sentence2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let sentence1 = "xD iP tqchblXgqvNVdi".to_string();
        let sentence2 = "FmtdCzv Gp YZf UYJ xD iP tqchblXgqvNVdi".to_string();
        let output = false;
        let result = are_sentences_similar(sentence1, sentence2);
        assert_eq!(result, output);
    }
}
