pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
    // brute force
    // tle on the last test case,
    // let mut chalk_limit = k;
    //
    // let n = chalk.len();
    // let mut i = 0;
    // let mut result = 0;
    // while i < n {
    //     chalk_limit -= chalk[i];
    //     if chalk_limit < 0 {
    //         result = i as i32;
    //         break;
    //     }
    //     i = (i + 1) % n;
    // }
    // result
    //
    // prefix sum
    // the sum of chalk could be mod by k and just iterate on the normal chalk array till we are
    // out of chalk, but we'll make it a little bit complex by using binary search

    let n = chalk.len();
    let mut prefix_sum = vec![0; n + 1];

    for i in 1..n + 1 {
        prefix_sum[i] = prefix_sum[i - 1] as i64 + chalk[i - 1] as i64;
    }

    dbg!(&prefix_sum);
    let sum = prefix_sum[n] as i64;
    let remaining_chalk = k as i64 % sum;
    dbg!(remaining_chalk);

    // F F F T T T T
    // we want the first T

    // ignore the 0 in the prefix sum

    let (mut low, mut high, mut ans) = (1, n, -1);

    while low <= high {
        let mid = low + (high - low) / 2;

        if (prefix_sum[mid] as i64) <= remaining_chalk {
            low = mid + 1;
        } else {
            high = mid - 1;
            ans = mid as i32;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let chalk = vec![5, 1, 5];
        let k = 22;
        let output = 0;
        let result = chalk_replacer(chalk, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let chalk = vec![3, 4, 1, 2];
        let k = 25;
        let output = 1;
        let result = chalk_replacer(chalk, k);
        assert_eq!(result, output);
    }
}
