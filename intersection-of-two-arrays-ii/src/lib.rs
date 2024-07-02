pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // brute force
    // let mut result = Vec::new();
    // let mut visited2 = vec![false; nums2.len()];
    // for n1 in nums1 {
    //     for (i, &n2) in nums2.iter().enumerate() {
    //         if n1 == n2 && !visited2[i] {
    //             result.push(n1);
    //             visited2[i] = true;
    //             break;
    //         }
    //     }
    // }
    // result
    // sort - binary search
    //
    let mut result = Vec::new();
    let mut nums1 = nums1.clone();
    nums1.sort_unstable();
    let mut nums2 = nums2.clone();
    nums2.sort_unstable();

    for &target in nums1.iter() {
        let mut l = 0i32;
        let mut r = nums2.len() as i32 - 1;

        while l <= r {
            let mid = (l + (r - l) / 2) as usize;

            if target == nums2[mid] {
                result.push(target);
                nums2.remove(mid);
                break;
            } else if nums2[mid] < target {
                l = mid as i32 + 1;
            } else {
                r = mid as i32 - 1;
            }
        }
    }
    result
    // freq array, works as a hashmap
    // let mut result = Vec::new();
    // let max = *nums1.iter().max().unwrap();
    // let mut freq1 = vec![0; max as usize + 1];
    // // let mut freq2 = vec![0; max as usize + 1];

    // for i in 0..nums1.len() {
    //     freq1[nums1[i] as usize] += 1;
    // }

    // dbg!(&freq1);

    // for num2 in nums2 {
    //     if num2 < freq1.len() as i32 && freq1[num2 as usize] >= 1 {
    //         freq1[num2 as usize] -= 1;
    //         result.push(num2);
    //     }
    // }

    // result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let output = vec![2, 2];
        let result = intersect(nums1, nums2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let output = vec![4, 9];
        let result = intersect(nums1, nums2);
        assert_eq!(result, output);
    }
}
