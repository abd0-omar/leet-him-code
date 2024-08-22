pub fn find_complement(num: i32) -> i32 {
    let num = format!("{num:b}");
    let result: String = num
        .chars()
        .map(|x| match x {
            '1' => '0',
            '0' => '1',
            _ => unreachable!(),
        })
        .collect();
    i32::from_str_radix(&result, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let num = 5;
        let output = 2;
        let result = find_complement(num);
        assert_eq!(result, output);
    }
}
