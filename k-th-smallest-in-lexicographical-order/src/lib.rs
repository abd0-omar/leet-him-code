pub fn find_kth_number(n: i32, k: i32) -> i32 {
    let mut curr = 1;
    let mut k = k - 1;

    while k > 0 {
        let step = count_steps(n, curr as i64, (curr + 1) as i64);
        if step <= k {
            curr += 1;
            k -= step;
        } else {
            curr *= 10;
            k -= 1;
        }
    }

    curr
}

fn count_steps(n: i32, mut prefix1: i64, mut prefix2: i64) -> i32 {
    let mut steps = 0;

    while prefix1 <= n as i64 {
        steps += (prefix2.min(n as i64 + 1) - prefix1) as i32;
        prefix1 *= 10;
        prefix2 *= 10;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 13;
        let k = 2;
        let output = 10;
        let result = find_kth_number(n, k);
        assert_eq!(result, output);
    }
}
