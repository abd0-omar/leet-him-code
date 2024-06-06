pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
    // 440
    // let mut freq = vec![0; max_num as usize + 1];
    hand.sort_unstable();
    // println!("freq={:?}", freq);
    // for num in hand {
    //     freq[num as usize] += 1;
    // }
    let mut freq = std::collections::HashMap::new();
    for &num in hand.iter() {
        freq.entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut freq_vec: Vec<_> = freq.into_iter().collect();
    freq_vec.sort_unstable();
    println!("freq_vec={:?}", freq_vec);

    'outer: loop {
        let mut count = group_size;
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
    fn it_works0() {
        // freq array
        // 0, 1, 2, 3, 4, 5, 6, 7, 8
        //[0, 1, 2, 2, 1, 0, 1, 1, 1]
        //[0, 0, 1, 1, 1, 0, 1, 1, 1]
        //[0, 0, 0, 0, 0, 0, 1, 1, 1]

        // 0, 1, 2, 3, 4, 5
        //[0, 1, 1, 1, 1, 1]
        //[0, 0, 0, 0, 1, 1]
        let hand = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
        let group_size = 3;
        let output = true;
        // Explanation: Alice's hand can be rearranged as [1,2,3],[2,3,4],[6,7,8]
        let result = is_n_straight_hand(hand, group_size);
        assert_eq!(result, output)
    }

    #[test]
    fn it_works1() {
        // 0, 1, 2
        //[0, 1, 1]
        //[0, 0, 0]
        //
        //it doesn't have to be group_size groups
        //but leetcode said that
        let hand = vec![2, 1];
        let group_size = 2;
        let output = true;
        // groups [1, 2], [empty]
        // so false
        let result = is_n_straight_hand(hand, group_size);
        assert_eq!(result, output)
    }

    #[test]
    fn it_works2() {
        // 0, 1, 2, 3
        //[0, 2, 2, 2]
        //[0, 1, 1, 2]
        //[0, 0, 0, 1]
        //
        // 1 | 2
        // 2 | 2
        // 3 | 2
        //
        let hand = vec![1, 1, 2, 2, 3, 3];
        let group_size = 2;
        let output = false;
        let result = is_n_straight_hand(hand, group_size);
        assert_eq!(result, output)
    }

    #[test]
    fn it_works3() {
        let hand = vec![1, 2, 3, 4, 5, 6];
        let group_size = 2;
        let output = true;
        let result = is_n_straight_hand(hand, group_size);
        assert_eq!(result, output)
    }

    #[test]
    fn it_works4() {
        let hand = vec![0, 1];
        let group_size = 2;
        let output = true;
        let result = is_n_straight_hand(hand, group_size);
        assert_eq!(result, output)
    }

    #[test]
    fn it_works5() {
        let hand = vec![
            233722108, 386144634, 221638886, 175028874, 408337082, 91410299, 793763682, 972910825,
            627251147, 135020779,
        ];
        let group_size = 2;
        let output = true;
        let result = is_n_straight_hand(hand, group_size);
        assert_eq!(result, output)
    }
}
