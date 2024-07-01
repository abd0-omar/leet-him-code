use std::collections::HashMap;

// TLE
fn main() {
    use std::io::stdin;
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let n: usize = line.trim().parse().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let s: Vec<String> = line.trim().chars().map(|c| c.to_string()).collect();
        // no zeros case
        // find the smallest pair and join them
        // then add all plusese
        let mut result = i128::MAX;
        if n == 2 {
            let s: String = s.iter().map(|s| &**s).collect();
            let s = s.parse::<i32>().unwrap();
            println!("{}", s);
            continue;
        }
        for i in 1..n {
            let pair1 = &s[i - 1];
            let pair2 = &s[i];
            let joined_pair = format!("{}{}", pair1, pair2);

            let mut y = Vec::with_capacity(n - 1);
            y.extend(&s[..(i - 1)]);
            y.push(&joined_pair);
            y.extend(&s[(i + 1)..]);

            let mut memory = HashMap::new();
            let cur_result = dp(&y, 0, 0, &mut memory);

            if cur_result == 0 {
                result = 0;
                break;
            }

            result = result.min(cur_result);
        }
        println!("{}", result);
    }
}

fn dp(
    new_s: &Vec<&String>,
    idx: usize,
    result: i128,
    memory: &mut HashMap<(usize, i128), i128>,
) -> i128 {
    if idx == new_s.len() - 1 {
        return result;
    }

    if let Some(&ret) = memory.get(&(idx, result)) {
        return ret;
    }

    let num1 = new_s[idx].parse::<i128>().unwrap();
    let num2 = new_s[idx + 1].parse::<i128>().unwrap();

    let mult_result = if idx == 0 { num1 * num2 } else { result * num2 };
    let add_result = if idx == 0 { num1 + num2 } else { result + num2 };

    let mult = dp(new_s, idx + 1, mult_result, memory);
    let add = dp(new_s, idx + 1, add_result, memory);

    let final_result = add.min(mult);

    memory.insert((idx, result), final_result);

    final_result
}
