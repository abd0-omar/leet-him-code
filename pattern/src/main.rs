fn main() {
    let nums1 = vec![1, 2, 3, 4];
    // Output: false
    // Explanation: There is no 132 pattern in the sequence.
    // Example 2:
    //
    let nums2 = vec![3, 1, 4, 2];
    // Output: true
    // Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
    // Example 3:
    //
    let nums3 = vec![-1, 3, 2, 0];
    // Output: true
    // Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].

    println!("{}", find132pattern(nums3));
}

pub fn find132pattern(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let n = nums.len();

    let mut first_stack: Vec<usize> = Vec::with_capacity(n);
    let mut second_stack = Vec::with_capacity(n);

    let mut rez = vec![i32::MIN; n];
    let mut zer = vec![i32::MAX; n];

    for i in 0..nums.len() {
        while !first_stack.is_empty() && nums[i] > nums[*first_stack.last().unwrap()] {
            rez[*first_stack.last().unwrap()] = nums[i];
            first_stack.pop();
        }

        while !second_stack.is_empty() && nums[i] < nums[*second_stack.last().unwrap()] {
            zer[*second_stack.last().unwrap()] = nums[i];
            second_stack.pop();
        }

        first_stack.push(i);
        second_stack.push(i);
    }

    // println!("DEBUGPRINT[12]: main.rs:44: rez={:#?}", rez);
    // println!("DEBUGPRINT[12]: main.rs:44: zer={:#?}", zer);
    // for i in 0..rez.len() {
    //     if rez[i] == i32::MIN && zer[i] != i32::MAX && i != 0 {
    //         return true;
    //     }
    // }

    for i in 1..rez.len() - 1 {
        if rez[i] == i32::MIN && rez[i - 1] > rez[i + 1] {
            return true;
        }
    }
    println!("DEBUGPRINT[13]: main.rs:52: rez={:#?}", rez);
    println!("DEBUGPRINT[13]: main.rs:52: rez={:#?}", zer);

    // println!("DEBUGPRINT[11]: main.rs:31: first_stack={:#?}", first_stack);
    // println!(
    //     "DEBUGPRINT[11]: main.rs:31: first_stack={:#?}",
    //     second_stack
    // );
    // println!("DEBUGPRINT[11]: main.rs:31: first_stack={:#?}", rez);
    // println!("DEBUGPRINT[11]: main.rs:31: first_stack={:#?}", zer);

    return false;
}

// pub fn find132pattern(nums: Vec<i32>) -> bool {
//     let mut nums_sorted = nums.clone();
//     nums_sorted.sort();
//
//     for num in nums {}
//     unimplemented!()
// }

// pub fn find132pattern(nums: Vec<i32>) -> bool {
//     let n = nums.len();
//
//     let mut i = 1;
//
//     while i < n - 1 {
//         let mut left: i32 = i as i32 - 1;
//         let mut right = i + 1;
//
//         let mut max = i32::MIN;
//         let mut min = i32::MAX;
//
//         while right < n {
//             if nums[right] > max && nums[i] > nums[right] {
//                 max = nums[right];
//             }
//             right += 1;
//         }
//
//         if max == i32::MIN {
//             i += 1;
//             continue;
//         }
//
//         while left >= 0 {
//             if nums[left as usize] < min && nums[left as usize] < max {
//                 min = nums[left as usize];
//                 break;
//             }
//             left -= 1;
//         }
//
//         if min != i32::MAX {
//             return true;
//         }
//
//         i += 1;
//     }
//     false
// }
