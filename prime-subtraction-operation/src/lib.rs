pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
    let sorted_primes_and_zero: Vec<i32> = (0..*nums.iter().max().unwrap())
        .filter(|num| match num {
            1 => false,
            0 | 2 => true,
            _ => {
                for div in 2..=((*num as f64).sqrt() as usize) {
                    if num % (div as i32) == 0 {
                        return false;
                    }
                }
                true
            }
        })
        .collect();
    dbg!(&sorted_primes_and_zero);
    let mut nums = nums;
    // dummy insert
    nums.insert(0, 0);
    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        dbg!(diff);
        if diff <= 0 {
            return false;
        }
        nums[i] -= binary_search_biggest_num_smaller_than(&sorted_primes_and_zero, diff);
    }
    true
}

fn binary_search_biggest_num_smaller_than(nums: &[i32], n: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let mid = (left + right) / 2;
        match nums[mid].cmp(&n) {
            std::cmp::Ordering::Equal => return nums[mid - 1],
            std::cmp::Ordering::Less => {
                left = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                right = mid - 1;
            }
        }
    }
    nums[right]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![4, 9, 6, 10];
        let output = true;
        let result = prime_sub_operation(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 2];
        let output = false;
        let result = prime_sub_operation(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![998, 2];
        let output = true;
        let result = prime_sub_operation(nums);
        assert_eq!(result, output);
    }
}
