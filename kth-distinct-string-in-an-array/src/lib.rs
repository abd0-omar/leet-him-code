pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    // O(n^2) twoo loops or a hashmap
    let mut hm: std::collections::HashMap<&String, i32> = std::collections::HashMap::new();
    let mut count = 0;
    let mut result = String::new();

    for element in arr.iter() {
        hm.entry(element)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for element in arr.iter() {
        if hm[&element] == 1 {
            count += 1;
            if count == k {
                result = element.to_owned();
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![
            "d".to_string(),
            "b".to_string(),
            "c".to_string(),
            "b".to_string(),
            "c".to_string(),
            "a".to_string(),
        ];
        let k = 2;
        let output = "a".to_string();
        let result = kth_distinct(arr, k);
        assert_eq!(result, output);
    }
}
