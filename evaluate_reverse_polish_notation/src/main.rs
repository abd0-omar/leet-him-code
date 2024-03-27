fn main() {
    let tokens = vec![
        "10".to_string(),
        "6".to_string(),
        "9".to_string(),
        "3".to_string(),
        "+".to_string(),
        "-11".to_string(),
        "*".to_string(),
        "/".to_string(),
        "*".to_string(),
        "17".to_string(),
        "+".to_string(),
        "5".to_string(),
        "+".to_string(),
    ];
    let mut stack: Vec<i32> = vec![];

    for c in tokens {
        // let num: i32 = c.parse().unwrap();

        match c.as_str() {
            "+" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y + x);
            }
            "-" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y - x);
            }
            "*" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y * x);
            }
            "/" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y / x);
            }
            _ => {
                stack.push(c.parse().unwrap());
            } // stack.push(num),
        }
    }
    println!("{:?}", stack);
}
