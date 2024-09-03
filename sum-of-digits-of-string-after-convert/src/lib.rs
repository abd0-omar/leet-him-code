pub fn get_lucky(s: String, k: i32) -> i32 {
    // brute-force
    // change all to their equivalent number
    let mut convert = vec![];
    for letter in s.into_bytes() {
        let mut letter = letter - b'a' + 1;
        //dbg!(letter);

        let mut number_reversed = vec![];
        while letter > 0 {
            let last = letter % 10;
            number_reversed.push(last);
            letter /= 10;
        }

        while let Some(num) = number_reversed.pop() {
            convert.push(num as u32);
        }
    }

    // go k times

    let mut sum = 0;
    let mut total = 0;
    for _ in 0..k {
        //dbg!(&convert);
        total = 0;
        for i in 0..convert.len() {
            sum += convert[i];
            total += convert[i];
        }
        // put sum into convert
        convert.clear();
        let mut number_reversed = vec![];
        while sum > 0 {
            let last = sum % 10;
            number_reversed.push(last);
            sum /= 10;
        }

        while let Some(num) = number_reversed.pop() {
            convert.push(num as u32);
        }
    }
    total as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "zbax".to_string();
        let k = 2;
        let output = 8;
        let result = get_lucky(s, k);
        assert_eq!(result, output);
    }
}
