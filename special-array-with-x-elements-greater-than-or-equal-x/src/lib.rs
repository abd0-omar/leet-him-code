pub fn special_array(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    // find first num >= me

    // for x in 0..1000 {
    for x in 0..=nums.len() {
        let first_num_bigger_than_me = search_first_num_bigger_than_me(&nums, x as i32);
        println!("x={:?}", x);
        println!("first_num_bigger_than_me={:?}", first_num_bigger_than_me);
        if let Some(first_bigger) = first_num_bigger_than_me {
            if x as i32 == nums.len() as i32 - first_bigger {
                return x as _;
            }
        }
    }

    -1
}

fn search_first_num_bigger_than_me(nums: &[i32], x: i32) -> Option<i32> {
    let mut l = 0i32;
    let mut r = nums.len() as i32 - 1;
    let mut ans = None;

    while l <= r {
        let mid = l + (r - l) / 2;

        if nums[mid as usize] >= x {
            r = mid - 1;
            ans = Some(mid);
        } else {
            l = mid + 1;
        }
    }
    ans
}

// pub fn special_array(mut nums: Vec<i32>) -> i32 {
//     nums.sort_unstable();
//     for x in 0..1000 {
//         let bigger_than_x = nums.iter().filter(|f| **f >= x as i32).count();
//         if bigger_than_x == x {
//             return x as _;
//         }
//     }
//     -1
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 5];
        let output = 2;
        let result = special_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![0, 0];
        let output = -1;
        let result = special_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        // 0, 0, 3, 4, 4
        let nums = vec![0, 4, 3, 0, 4];
        let output = 3;
        let result = special_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let nums = vec![3, 6, 7, 7, 0];
        let output = -1;
        let result = special_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let nums = vec![1, 1, 2];
        let output = -1;
        let result = special_array(nums);
        assert_eq!(result, output);
    }
}
