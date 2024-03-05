fn main() {
    println!("Hello, world!");
    //     Example 1:
    //
    let num = "6777133339".to_string();
    // Output: "777"
    println!("{}", largest_good_integer(num));
}

pub fn largest_good_integer(num: String) -> String {
    let num = num.chars().collect::<Vec<_>>();
    let mut count = 0;
    let mut max = None;
    for i in 1..num.len() {
        if num[i] == num[i - 1] {
            count += 1;
        } else {
            count = 0;
        }
        if count == 2 {
            println!("{}", num[i]);
            match max {
                Some(m) => {
                    if num[i] > m {
                        max = Some(num[i]);
                    }
                }
                None => {
                    max = Some(num[i]);
                }
            }
        }
    }

    // match max {
    //     Some(m) => m.to_string().repeat(3),
    //     None => String::new(),
    // }
    max.map_or(String::new(), |m| m.to_string().repeat(3))
}
