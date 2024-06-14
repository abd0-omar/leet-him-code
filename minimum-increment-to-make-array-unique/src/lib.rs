pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    // freq hm
    // if the number duplicated, search for the next greatest number
    // that could be using a monotonic stack, but keep it for later
    // let mut hm = std::collections::HashMap::new();
    // for &num in nums.iter() {
    //     hm.entry(num)
    //         .and_modify(|counter| *counter += 1)
    //         .or_insert(1);
    // }
    // dbg!(&hm);
    // let mut max_num = *nums.iter().max().unwrap();
    // let mut result = 0;
    // for num in nums {
    //     let freq = *hm.get(&num).unwrap();
    //     if freq > 1 {
    //         dbg!(num);
    //         // search for the next empty spot
    //         // starting from num+1 to the end
    //         let mut count = 0;
    //         dbg!(num + 1..=max_num);
    //         for i in num + 1..=max_num + 1 {
    //             count += 1;
    //             if hm.get(&i).is_none() {
    //                 if i == max_num + 1 {
    //                     max_num += 1;
    //                 }
    //                 hm.insert(i, 1);
    //                 let (temp_num, temp_freq) = hm.remove_entry(&num).unwrap();
    //                 dbg!(&hm);
    //                 if temp_freq > 1 {
    //                     hm.insert(temp_num, temp_freq - 1);
    //                 }
    //                 dbg!(&hm);
    //                 result += count;
    //                 break;
    //             }
    //         }
    //     }
    // }
    // result
    // time limit exceeded
    // now we can improve after we got an idea how it could be solved
    // maybe a freq array could work even with large input
    // let mut max = *nums.iter().max().unwrap();
    // let mut freq = vec![0; max as usize + 1];
    // let mut visited = std::collections::HashSet::new();
    // for &num in nums.iter() {
    //     freq[num as usize] += 1;
    // }
    // let mut result = 0;
    // for num in nums {
    //     // if not unique
    //     if visited.contains(&num) {
    //         continue;
    //     }
    //     if freq[num as usize] > 1 {
    //         // search for the next empty spot
    //         for i in num + 1..=max {
    //             if visited.contains(&i) {
    //                 continue;
    //             }
    //             if freq[i as usize] == 0 {
    //                 dbg!(num);
    //                 dbg!(i);
    //                 result += i - num;
    //                 visited.insert(i);
    //                 freq[num as usize] -= 1;
    //                 break;
    //             }
    //         }
    //     }
    // }
    // result
    // it was very brute force after all
    nums.sort_unstable();
    let n = nums.len();
    let mut result = 0;
    for i in 1..n {
        match nums[i].cmp(&nums[i - 1]) {
            std::cmp::Ordering::Less => {
                let new_num = nums[i - 1] - nums[i] + 1;
                result += new_num;
                nums[i] = nums[i - 1] + 1;
            }
            std::cmp::Ordering::Equal => {
                let new_num = nums[i] + 1;
                result += 1;
                nums[i] = new_num;
            }
            std::cmp::Ordering::Greater => (),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 2, 1, 2, 1, 7];
        let output = 6;
        let result = min_increment_for_unique(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 2];
        let output = 1;
        let result = min_increment_for_unique(nums);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works2() {
        let nums = vec![2, 2, 2, 1];
        let output = 3;
        let result = min_increment_for_unique(nums);
        assert_eq!(result, output);
    }
}
