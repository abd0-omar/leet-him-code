fn main() {
    println!("Hello, world!");
    let bank = vec![
        "011001".to_string(),
        "000000".to_string(),
        "010100".to_string(),
        "001000".to_string(),
    ];
    println!("{}", number_of_beams(bank));
}

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let bank: Vec<Vec<char>> = bank.iter().map(|x| x.chars().collect()).collect();
    let m = bank.len();
    let n = bank[0].len();
    let mut res = 0;
    let mut prev = None;
    for i in 0..m {
        let mut sum = 0;
        let mut is_zeros = false;
        for j in 0..n {
            if all_zeros(&bank[i], j) {
                is_zeros = true;
                break;
            }

            if bank[i][j] == '1' {
                sum += 1;
            }
        }
        if is_zeros {
            continue;
        }
        if let Some(p) = prev {
            res += p * sum;
            // println!("res={:?}", res);
            // println!("sum={:?}", sum);
            // println!("p={:?}", p);
            prev = Some(sum);
        } else {
            prev = Some(sum);
        }
        match prev {
            Some(p) => {
                res += p * sum;
                prev = Some(sum)
            }
            // firs time
            None => prev = Some(sum),
        }
    }
    res
}

fn all_zeros(vec: &Vec<char>, j: usize) -> bool {
    if j > 0 {
        return false;
    }

    for v in vec {
        if v != &'0' {
            return false;
        }
    }
    true
}
