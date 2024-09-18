pub fn largest_number(nums: Vec<i32>) -> String {
    // let the string comparison handle it it's self
    let mut numbers: Vec<String> = nums.into_iter().map(|x| x.to_string()).collect();

    numbers.sort_by(|a, b| {
        let a = format!("{}{}", a, b);
        let b = format!("{}{}", b, a);
        b.cmp(&a)
    });

    if numbers[0] == "0" {
        return "0".to_string();
    }

    numbers.concat()
}

// pub fn largest_number(nums: Vec<i32>) -> String {
//     // compare the first digit,
//     // if there is a tie,
//     // compare with the second digit and so on
//     // and having a 0 as a second digit is lower than having nothing
//     let mut numbers = Vec::with_capacity(nums.len());
//     for num in nums {
//         let first_digit = get_first_digit(num);
//         dbg!(first_digit);
//         // if there is a tie,
//         numbers.push(num);
//     }
//     numbers.sort_unstable_by(|&a, &b| {
//         let f = get_first_digit(b).cmp(&get_first_digit(a));
//         let x = match f {
//             std::cmp::Ordering::Equal => {
//                 // compare with second number
//                 // it could be done recursively
//                 // we want the bigger second number
//                 Some(dfs(a, b))
//             }
//             _ => None,
//         };
//         match x {
//             Some(x) => x,
//             None => f,
//         }
//     });
//
//     dbg!(&numbers);
//
//     let result: String = numbers.iter().map(|x| x.to_string()).collect();
//     let mut new_result = result.clone();
//     for (i, &num) in result.as_bytes().iter().enumerate() {
//         if num == b'0' {
//             new_result.remove(i);
//             new_result.push(char::from_digit(0, 10).unwrap())
//         }
//     }
//     // 0123456
//     // 9534033
//     // 9534330
//
//     dbg!(&result);
//     dbg!(&new_result);
//     new_result
// }
//
// fn dfs(a: i32, b: i32) -> std::cmp::Ordering {
//     dbg!("happened");
//     dbg!(a);
//     dbg!(b);
//     if a / 10 == 0 {
//         dbg!("a is zero");
//         dbg!(a);
//     } else if b / 10 == 0 {
//         dbg!("b is zero");
//         dbg!(b);
//     }
//     // dbg!(a);
//     // dbg!(b);
//     if a.cmp(&b) == std::cmp::Ordering::Equal {
//         let n_a = a / 10;
//         let n_b = b / 10;
//         dfs(n_a, n_b)
//     } else {
//         b.cmp(&a)
//     }
// }
//
// fn get_first_digit(mut num: i32) -> i32 {
//     while num >= 10 {
//         num /= 10;
//     }
//
//     num
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 30, 34, 5, 9];
        let output = "9534330".to_string();
        let result = largest_number(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![10, 2];
        let output = "210".to_string();
        let result = largest_number(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![111311, 1113];
        let output = "1113111311".to_string();
        let result = largest_number(nums);
        assert_eq!(result, output);
    }
}
