fn main() {
    println!("Hello, world!");
    let matches = vec![
        vec![1, 3],
        vec![2, 3],
        vec![3, 6],
        vec![5, 6],
        vec![5, 7],
        vec![4, 5],
        vec![4, 8],
        vec![4, 9],
        vec![10, 4],
        vec![10, 9],
    ];
    // Output: [[1,2,10],[4,5,7,8]]
    println!("{:?}", find_winners(matches));
}

use std::collections::HashMap;
pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut hm0 = HashMap::new();
    let mut hm1 = HashMap::new();
    for macht in matches {
        let won = macht[0];
        let lost = macht[1];
        *hm0.entry(won).or_insert(0) += 1;
        *hm1.entry(lost).or_insert(0) += 1;
    }
    println!("hm0={:?}", hm0);
    println!("hm1={:?}", hm1);
    let mut answer: Vec<Vec<i32>> = Vec::with_capacity(2);
    // let mut hm0_iter = hm0.into_iter();
    // let mut hm1_iter = hm1.into_iter();
    // loop {
    //     match (hm0_iter.next(), hm1_iter.next()) {
    //         (None, None) => return answer,
    //         (None, Some(_)) => ,
    //         (Some(_), None) => todo!(),
    //         (Some(fir), Some(sec)) => {
    //             if fir.1 > 0 &&
    //         },
    //     }
    // }

    let mut rez0: Vec<i32> = vec![];
    let mut rez1: Vec<i32> = vec![];
    for &key in hm0.keys() {
        // match hm1.get(&key) {
        //     Some(val1) => {
        //         if val1 == &1 {
        //             rez1.push(key);
        //         }
        //     }
        //     None => rez0.push(key),
        // }
        if !hm1.contains_key(&key) {
            rez0.push(key);
        }
    }

    for (&key, &val) in hm1.iter() {
        if val == 1 {
            rez1.push(key);
        }
    }

    rez0.sort_unstable();
    rez1.sort_unstable();
    answer.push(rez0);
    answer.push(rez1);
    answer
}
