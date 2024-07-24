pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
    // sort by freq asc
    // if equal, sort by num desc
    /*
    2, 3, 1, 3, 2
    freq | num
    1 | 1
    2 | [3, 2]
    so a hm <freq, binaryheap num>
    */
    use std::collections::HashMap;
    // let hm = HashMap::new();
    let mut normal_freq = HashMap::new();
    for &num in nums.iter() {
        normal_freq
            .entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    /*
    normal freq
    num | freq
    1 | 1
    2 | 2
    3 | 2
    */
    /*
    tuple for easy sorting
    (freq, num)
    (1, 1), (2, 2), (2, 3)
    and I could use the eq trait but let's do something simpler
    two options
    make it a tuple then sort the equal nums
    insertion sort
    */
    // Insertion sort based on frequency and then by number descending
    // for i in 1..nums.len() {
    //     let key_num = nums[i];
    //     let key_freq = *normal_freq.get(&key_num).unwrap();
    //     let mut j = i as isize - 1;

    //     while j >= 0 {
    //         let current_num = nums[j as usize];
    //         let current_freq = *normal_freq.get(&current_num).unwrap();

    //         if current_freq > key_freq || (current_freq == key_freq && current_num < key_num) {
    //             nums[j as usize + 1] = nums[j as usize];
    //             j -= 1;
    //         } else {
    //             break;
    //         }
    //     }

    //     nums[(j + 1) as usize] = key_num;
    // }
    // nums
    //
    // sort by way
    //
    nums.sort_by(|&a, &b| {
        let freq_a = normal_freq.get(&a).unwrap();
        let freq_b = normal_freq.get(&b).unwrap();
        if freq_a == freq_b {
            b.cmp(&a)
        } else {
            freq_a.cmp(&freq_b)
        }
    });
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 3, 1, 3, 2];
        let output = vec![1, 3, 3, 2, 2];
        let result = frequency_sort(nums);
        assert_eq!(result, output);
    }
}
