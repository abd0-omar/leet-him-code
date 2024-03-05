fn main() {
    println!("Hello, world!");
    let n = 4;
    let trust = vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]];
    println!("{}", find_judge(n, trust));
}

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    if trust.is_empty() {
        if n == 1 {
            return 1;
        } else {
            return -1;
        }
    }

    let n = n as usize;
    let mut hm = std::collections::HashMap::new();

    for t in trust.iter() {
        // let f = t[1];
        // println!("f={:?}", f);
        *hm.entry(t[1]).or_insert(0) += 1;
    }

    // let mut vec = hm.iter().collect::<Vec<_>>();
    let max = hm.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    // println!("max={:?}", max);

    // vec.sort_unstable_by(|x, y| y.1.cmp(x.1));
    // println!("vec={:?}", vec);

    let town_judge = max.0;
    let town_judge_trust_num = max.1;

    if *town_judge_trust_num < n - 1 {
        return -1;
    }

    for t in trust {
        // if the judge trust someone, return -1;
        if town_judge == &t[0] {
            return -1;
        }
    }

    *town_judge
}
