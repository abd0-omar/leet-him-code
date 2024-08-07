pub fn number_to_words(mut num: i32) -> String {
    if num == 0 {
        return "Zero".to_string();
    }
    let mut res = vec![];
    let mut third = -1;
    while num > 0 {
        let val = num % 1000;
        num = num / 1000;
        third += 1;

        let mut words = vec![];

        thousand_to_words(val, &mut words);

        if words.len() == 0 {
            continue;
        }

        match third {
            0 => {}
            1 => words.push("Thousand"),
            2 => words.push("Million"),
            3 => words.push("Billion"),
            _ => unreachable!(),
        }

        res.push(words);
    }
    res.into_iter()
        .rev()
        .map(|w| w.join(" "))
        // .flatten()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn thousand_to_words(mut val: i32, words: &mut Vec<&'static str>) {
    if val == 0 {
        return;
    }

    if val / 100 > 0 {
        words.push(digit_to_word(val / 100));
        words.push("Hundred");
    }
    val = val % 100;

    if val == 0 {
        return;
    } else if val < 10 {
        words.push(digit_to_word(val));
    } else if val < 20 {
        words.push(teen_to_word(val));
    } else {
        words.push(ten_to_word(val / 10));
        if val % 10 != 0 {
            words.push(digit_to_word(val % 10));
        }
    }
}

pub fn ten_to_word(mut val: i32) -> &'static str {
    match val {
        2 => "Twenty",
        3 => "Thirty",
        4 => "Forty",
        5 => "Fifty",
        6 => "Sixty",
        7 => "Seventy",
        8 => "Eighty",
        9 => "Ninety",
        _ => unreachable!("{}", val),
    }
}

pub fn teen_to_word(mut val: i32) -> &'static str {
    match val {
        10 => "Ten",
        11 => "Eleven",
        12 => "Twelve",
        13 => "Thirteen",
        14 => "Fourteen",
        15 => "Fifteen",
        16 => "Sixteen",
        17 => "Seventeen",
        18 => "Eighteen",
        19 => "Nineteen",
        _ => unreachable!("{}", val),
    }
}

pub fn digit_to_word(mut val: i32) -> &'static str {
    match val {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => unreachable!("{}", val),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let num = 1234567;
        let output =
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string();
        let result = number_to_words(num);
        assert_eq!(result, output);
    }
}
