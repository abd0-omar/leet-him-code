pub fn compare_version(version1: String, version2: String) -> i32 {
    // compare before '.' and after the '.'
    let mut parts1 = version1.split('.');
    let mut parts2 = version2.split('.');
    let mut result = 0i32;
    loop {
        match (parts1.next(), parts2.next()) {
            (None, None) => break,
            (None, Some(num2)) => {
                if result != 0 {
                    return -1;
                }
                dbg!(num2.parse::<i32>().unwrap());
                if num2.parse::<i32>().unwrap() > 0 {
                    return -1;
                }
            }
            (Some(num1), None) => {
                if result != 0 {
                    return 1;
                }
                dbg!(num1.parse::<i32>().unwrap());
                if num1.parse::<i32>().unwrap() > 0 {
                    return 1;
                }
            }
            (Some(num1), Some(num2)) => {
                let num1: i32 = num1.parse().unwrap();
                let num2: i32 = num2.parse().unwrap();
                if num1 > num2 || num2 > num1 {
                    result = result + (num1 - num2);
                    break;
                }
            }
        }
    }

    if result.is_positive() {
        1
    } else if result.is_negative() {
        -1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let version1 = "1.01".to_string();
        let version2 = "1.001".to_string();
        let output = 0;
        let result = compare_version(version1, version2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let version1 = "1.0".to_string();
        let version2 = "1.0.0".to_string();
        let output = 0;
        let result = compare_version(version1, version2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let version1 = "0.1".to_string();
        let version2 = "1.0".to_string();
        let output = -1;
        let result = compare_version(version1, version2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let version1 = "1.0.1".to_string();
        let version2 = "1".to_string();
        let output = 1;
        let result = compare_version(version1, version2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let version1 = "1.00.1".to_string();
        let version2 = "1.0.2".to_string();
        let output = -1;
        let result = compare_version(version1, version2);
        assert_eq!(result, output);
    }
}
