fn main() {
    println!("Hello, world!");
    // Example 1:

    let strs = vec![
        "flight".to_string(),
        "flow".to_string(),
        "flower".to_string(),
    ];
    let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    // Output: "fl"
    println!("{}", longest_common_prefix(strs));
}
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut index = 0;
    let mut result = String::new();

    //for str in strs {
    //   let mut idx = 0;
    //     for byte in str.as_bytes() {
    //         if
    //     }
    // }

    // let mut i = 1;
    // let mut j = 0;
    // while i < strs.len() {
    //     if strs[i][j..j+1] {

    //     }
    // }

    'outer: loop {
        for i in 1..strs.len() {
            //if strs[i].get(&index).is_none(){
            if strs[i].get(index..index + 1).is_none() {
                break 'outer;
            }

            if strs[i].get(index..index + 1).is_some()
                && strs[i - 1].get(index..index + 1).is_some()
                && strs[i].get(index..index + 1).unwrap()
                    != strs[i - 1].get(index..index + 1).unwrap()
            {
                break 'outer;
            }
        }
        index += 1;
        let option_push = strs[0].get(index - 1..index);
        if let Some(push) = option_push {
            result.push_str(push);
        } else {
            break;
        }
    }
    result
}
