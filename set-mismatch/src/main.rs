fn main() {
    println!("Hello, world!");
    // Example 1:

    let nums = vec![1, 2, 2, 4];
    // Output: [2,3]
    // Example 2:

    println!("{:?}", find_error_nums(nums));
    let nums = vec![1, 1];
    // Output: [1,2]
    println!("{:?}", find_error_nums(nums));
    let nums = vec![2, 2];
    println!("{:?}", find_error_nums(nums));
    let nums = vec![1, 5, 3, 2, 2, 7, 6, 4, 8, 9];
    println!("{:?}", find_error_nums(nums));
    let nums = vec![3, 2, 3, 4, 6, 5];
    println!("{:?}", find_error_nums(nums));
}

pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();
    let mut rez = vec![-1, -1];

    // if nums.len() == 2 {
    //     if nums[0] == 1 && nums[1] == 1 {
    //         rez[](1);
    //         rez.push(2);
    //     }
    //     if nums[0] != 1 && nums[1] == nums[0] {
    //         rez.push(nums[0]);
    //         rez.push(nums[0] - 1);
    //     }
    //     return rez;
    // }

    let mut seen = -1;
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            rez[0] = nums[i];
            seen = i as i32;
            break;
        }
    }

    let mut counter = 1;
    for i in 0..nums.len() {
        if nums[i] != counter {
            if seen == i as i32 {
                // counter += 2;
                continue;
            }
            if counter < nums[i] {
                rez[1] = nums[i] - 1;
            } else {
                rez[1] = nums[i] + 1;
            }
            break;
        }
        counter += 1;
    }

    if rez[1] == -1 {
        rez[1] = nums.len() as i32;
    }

    rez
}

