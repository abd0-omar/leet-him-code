pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    // search if the mid num (from binary search) there are m boquets could be made from k adjacent flowers in O(n)
    let mut l = *bloom_day.iter().min().unwrap();
    let mut r = *bloom_day.iter().max().unwrap();
    let mut ans = -1;
    while l <= r {
        // try random day
        let mid = l + (r - l) / 2;
        // F F F F T T T T
        // we want the first true
        dbg!(&mid);
        if possible(&bloom_day, mid, m, k) {
            r = mid - 1;
            ans = mid;
        } else {
            l = mid + 1;
        }
    }
    ans
}

fn possible(bloom_day: &[i32], mid: i32, m: i32, k: i32) -> bool {
    let mut k_counter = 0;
    let mut m_counter = 0;
    for &bloom in bloom_day {
        if bloom <= mid {
            k_counter += 1;
        } else {
            k_counter = 0;
        }
        if k_counter == k {
            m_counter += 1;
            k_counter = 0;
        }
        if m_counter == m {
            return true;
        }
        dbg!(k_counter);
        dbg!(m_counter);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let bloom_day = vec![1, 10, 3, 10, 2];
        let m = 3;
        let k = 1;
        let output = 3;
        let result = min_days(bloom_day, m, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let bloom_day = vec![7, 7, 7, 7, 12, 7, 7];
        let m = 2;
        let k = 3;
        let output = 12;
        let result = min_days(bloom_day, m, k);
        assert_eq!(result, output);
        // Explanation: We need 2 bouquets each should have 3 flowers.
        // Here is the garden after the 7 and 12 days:
        // After day 7: [x, x, x, x, _, x, x]
        // We can make one bouquet of the first three flowers that bloomed. We cannot make another bouquet from the last three flowers that bloomed because they are not adjacent.
        // After day 12: [x, x, x, x, x, x, x]
        // It is obvious that we can make two bouquets in different ways.
    }
}
