pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut count = 0;
    for log in logs {
        match log.as_str() {
            "../" => {
                if count > 0 {
                    count -= 1;
                }
            }
            "./" => (),
            _ => count += 1,
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let logs = vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string(),
        ];
        let output = 2;
        let result = min_operations(logs);
        assert_eq!(result, output);
    }
}
