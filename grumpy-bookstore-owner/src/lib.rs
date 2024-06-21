pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    // [1, 0, 1, 2, 1, 1, 7, 5]
    // [0, 1, 0, 1, 0, 1, 0, 1]
    // maybe prefix sum the first array
    // [0, 1, 1, 2, 4, 5, 6, 13, 18]
    // then take all the three mins window in one loop
    // [0, 1, 0, 1, 0, 1, 0, 1]
    //  _______
    //  ans is 2 - 0, (i + minutes) - i, flip this part if possible
    //  sum of the whole area without the window, 18 - 2 -> 16, last - (i + minutes)
    //  total sum, sum of the whole area without the window + ans of flips if possible
    //  put it in our max_sum
    //     _______
    //        _______
    //
    // if that didn't work do brute force, like you should've!, premature optimization
    // it worked!, yeah it took time but it worked, I think I still should've done brute force first
    // I know next time I'll ignore the brute force and try my "clever" solution, but yeah you should always come up with the brute force solution first

    // this is not the most "optimal" but it works.
    // we don't need prefix_sum
    // just a normal fixed sliding window problem, you should revise that from that one youtube video
    let n = customers.len();
    let mut prefix_sum = vec![0; n + 1];
    for i in 1..n + 1 {
        if grumpy[i - 1] == 1 {
            prefix_sum[i] = prefix_sum[i - 1];
        } else {
            prefix_sum[i] = prefix_sum[i - 1] + customers[i - 1];
        }
    }
    dbg!(&prefix_sum);

    // loop on every `minutes` window
    let mut max_sum = 0;
    for i in 0..n - minutes as usize + 1 {
        dbg!(i);
        // range
        let range_grumpy = &grumpy[i..i + minutes as usize];
        dbg!(&range_grumpy);
        // let mut range_customers = &customers[i..minutes as usize];
        let mut range_customers = vec![0; minutes as usize];
        range_customers.clone_from_slice(&customers[i..i + minutes as usize]);
        // flip the customers if possible
        // let mut idx = 0;
        // for j in i..i + minutes as usize {
        //     if grumpy[j] == 0 {
        //         range_customers[idx] = 0
        //     }
        //     idx += 1;
        // }
        println!("range_customers after flipping");
        dbg!(&range_customers);
        let ans_of_flips: i32 = range_customers.iter().sum();
        let last = *prefix_sum.last().unwrap();
        let after_minutes = prefix_sum[i + minutes as usize];
        let sum_last_part = last - after_minutes;
        dbg!(sum_last_part);
        // sum first part before the window
        let sum_first_part = if i != 0 { prefix_sum[i] } else { 0 };
        dbg!(sum_first_part);
        let sum_whole_without_window = sum_first_part + sum_last_part;
        dbg!(sum_whole_without_window);
        let sum_all = sum_whole_without_window + ans_of_flips;
        dbg!(sum_all);
        max_sum = max_sum.max(sum_all);
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let customers = vec![1, 0, 1, 2, 1, 1, 7, 5];
        let grumpy = vec![0, 1, 0, 1, 0, 1, 0, 1];
        let minutes = 3;
        let output = 16;
        // Explanation: The bookstore owner keeps themselves not grumpy for the last 3 minutes.
        // The maximum number of customers that can be satisfied = 1 + 1 + 1 + 1 + 7 + 5 = 16.
        let result = max_satisfied(customers, grumpy, minutes);
        assert_eq!(result, output);
    }
}
