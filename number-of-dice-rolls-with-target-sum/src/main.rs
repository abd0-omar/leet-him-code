use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    // let n = 30;
    // let k = 30;
    // let target = 500;
    // // Output: 222616187
    // println!("{}", num_rolls_to_target(n, k, target));
    let n = 2;
    let k = 6;
    let target = 7;
    // Output: 6
    println!("{}", num_rolls_to_target(n, k, target));
    // let n = 1;
    // let k = 6;
    // let target = 3;
    // // Output: 1
    // println!("{}", num_rolls_to_target(n, k, target));
}

const MOD: i32 = 1_000_000_007;

pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    let v: Vec<i32> = (1..=k).collect::<Vec<i32>>();
    let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    _num_rolls_to_target(target, &v, n, &mut memo)
}

pub fn _num_rolls_to_target(
    target: i32,
    v: &Vec<i32>,
    n: i32,
    memo: &mut HashMap<(i32, i32), i32>,
) -> i32 {
    if target == 0 && n == 0 {
        return 1;
    }

    if n == 0 {
        return 0;
    }

    if let Some(&cached_result) = memo.get(&(target, n)) {
        return cached_result;
    }

    let mut res = 0;
    for &num in v {
        if target >= num {
            res += _num_rolls_to_target(target - num, v, n - 1, memo);
            res %= MOD;
        }
    }

    memo.insert((target, n), res);
    res
}
