pub fn count_seniors(details: Vec<String>) -> i32 {
    // let mut count = 0;
    // for detail in details {
    //     let age = &detail[11..=12];
    //     let number: i32 = age.parse().unwrap();
    //     if number > 60 {
    //         count += 1;
    //     }
    // }
    // count
    details
        .iter()
        .filter(|&detail| &detail[11..=12].parse::<i32>().unwrap() > &60)
        .count() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let details = vec![
            "7868190130M7522".to_string(),
            "5303914400F9211".to_string(),
            "9273338290F4010".to_string(),
        ];
        let output = 2;
        let result = count_seniors(details);
        assert_eq!(result, output);
    }
}
