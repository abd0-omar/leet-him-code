fn main() {
    println!("Hello, world!");
    // Example 1:

    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    // Output: [[2,2,3],[7]]
    // Explanation:
    // 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
    // 7 is a candidate, and 7 = 7.
    // These are the only two combinations.
    println!("{:?}", combination_sum(candidates, target));

    let candidates = vec![2, 3, 5];
    let target = 8;
    println!("{:?}", combination_sum(candidates, target));
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut vecz = std::collections::HashSet::new();
    _combination_sum(&candidates, &target, vec![], &mut vecz);
    // vecz.sort_unstable();
    println!("vecz={:?}", vecz);
    vecz.iter().cloned().collect()
}

pub fn _combination_sum(
    candidates: &Vec<i32>,
    target: &i32,
    mut small_vec: Vec<i32>,
    big_vecz: &mut std::collections::HashSet<Vec<i32>>,
) {
    let sum_small_vec: i32 = small_vec.iter().sum();
    if target == &sum_small_vec {
        println!("small_vec={:?}", small_vec);
        println!("sum_small_vec={:?}", sum_small_vec);
        if sum_small_vec == *target {
            small_vec.sort_unstable();
            big_vecz.insert(small_vec.clone());
        }
        return;
    }

    if target < &sum_small_vec {
        return;
    }

    for &cand in candidates.iter() {
        small_vec.push(cand);
        _combination_sum(candidates, target, small_vec.clone(), big_vecz);
        small_vec.pop();
    }

    // println!("small_vec={:?}", small_vec);
}
