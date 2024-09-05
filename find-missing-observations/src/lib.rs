pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    // binary search for the total distribution sum
    let mut sum = 0;
    let m = rolls.len();

    for roll in rolls.iter() {
        sum += roll;
    }

    let mut l = 1;
    // max we can get from the rolls left
    let mut r = n * 6;
    let mut total_sum_of_rolls_left = -1;

    while l <= r {
        let mid = l + (r - l) / 2;

        if (mid + sum) as f64 / (n + m as i32) as f64 == mean as f64 {
            total_sum_of_rolls_left = mid;
            break;
        }
        if (mid + sum) / (n + m as i32) < mean {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    // if total sum of the left rolls is less than n (we can't distribute it to 1s)
    // or
    // total sum > n * 6 ( we can't distribute it to 6s)
    if total_sum_of_rolls_left < n || total_sum_of_rolls_left > n * 6 {
        return Vec::new();
    }

    dbg!(&total_sum_of_rolls_left); // 12

    // 0
    let distribute_rolls_left = total_sum_of_rolls_left % n;
    // can be all equally distributed in our case

    let mut result = vec![total_sum_of_rolls_left / n; n as usize];

    // if it's not equally distribued, (there's a remainder)
    for i in 0..distribute_rolls_left {
        result[i as usize] += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let rolls = vec![3, 2, 4, 3];
        let mean = 4;
        let n = 2;
        let output = vec![6, 6];
        let result = missing_rolls(rolls, mean, n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let rolls = vec![1, 5, 6];
        let mean = 3;
        let n = 4;
        let output = vec![2, 3, 2, 2];
        let result = missing_rolls(rolls, mean, n);
        assert_eq!(result, output);
    }
}
