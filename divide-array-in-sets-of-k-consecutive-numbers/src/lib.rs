pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
    // 440
    // let mut freq = vec![0; max_num as usize + 1];
    // nums.sort_unstable();
    // println!("freq={:?}", freq);
    // for num in hand {
    //     freq[num as usize] += 1;
    // }
    let mut freq = std::collections::HashMap::new();
    for &num in nums.iter() {
        freq.entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut freq_vec: Vec<_> = freq.into_iter().collect();
    freq_vec.sort_unstable();
    // println!("freq_vec={:?}", freq_vec);

    'outer: loop {
        let mut count = k;
        let mut prev: Option<i32> = None;
        let mut all_zeros = true;
        for (num, freq) in freq_vec.iter_mut() {
            if *freq == 0 {
                continue;
            }
            all_zeros = false;

            if let Some(p) = prev {
                if *freq >= 1 && p == *num - 1 {
                    *freq -= 1;
                    count -= 1;
                    prev = Some(*num);
                    if count == 0 {
                        continue 'outer;
                    }
                }
            } else {
                if *freq >= 1 {
                    *freq -= 1;
                    count -= 1;
                    prev = Some(*num);
                    if count == 0 {
                        continue 'outer;
                    }
                }
            }
        }

        if all_zeros {
            break;
        }
        return false;
    }
    true

    // 'outer: loop {
    //     let mut count = group_size;
    //     let mut prev: Option<usize> = None;
    //     let mut all_zeros = true;
    //     let mut did_do_an_outer = false;
    //     for j in 0..freq.len() {
    //         println!("j={:?}", j);
    //         println!("freq={:?}", freq);
    //         if *freq.get(&hand[j]).unwrap() == 0 {
    //             continue;
    //         }
    //         all_zeros = false;
    //         if let Some(p) = prev {
    //             if *freq.get(&hand[j]).unwrap() >= 1 && p == j - 1 {
    //                 *freq.get_mut(&hand[j]).unwrap() -= 1;
    //                 prev = Some(j);
    //                 count -= 1;
    //                 if count == 0 {
    //                     did_do_an_outer = true;
    //                     continue 'outer;
    //                 }
    //             } else {
    //                 continue;
    //             }
    //         } else {
    //             if *freq.get(&hand[j]).unwrap() >= 1 {
    //                 *freq.get_mut(&hand[j]).unwrap() -= 1;
    //                 prev = Some(j);
    //                 count -= 1;
    //                 if count == 0 {
    //                     did_do_an_outer = true;
    //                     continue 'outer;
    //                 }
    //             }
    //         }
    //     }
    //
    //     if all_zeros {
    //         break;
    //     }
    //     return false;
    // }
    // println!("freq={:?}", freq);
    // for f in freq {
    //     if f != 0 {
    //         // println!("wow");
    //         return false;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
