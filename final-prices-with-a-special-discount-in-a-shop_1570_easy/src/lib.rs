// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        // input too low brute-force would work
        //
        // next smaller element, same as https://leetcode.com/problems/daily-temperatures/
        // monotonic stack ascendening (increasing)
        // stack: 8
        // stack: None
        // stack: 4
        // stack: 4, 6
        // stack: 2
        // stack: 2, 3
        let n = prices.len();
        let mut result = prices.clone();
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(n);
        for (idx, price) in prices.into_iter().enumerate() {
            while !stack.is_empty() && stack.last().unwrap().1 >= price {
                let (top_idx, top_price) = stack.pop().unwrap();
                result[top_idx] = top_price - price;
            }
            stack.push((idx, price));
        }
        // could've also made stack just push indexes only
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let prices = vec![8, 4, 6, 2, 3];
        let output = vec![4, 2, 4, 2, 3];
        let result = Solution::final_prices(prices);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let prices = vec![1, 2, 3, 4, 5];
        let output = vec![1, 2, 3, 4, 5];
        let result = Solution::final_prices(prices);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let prices = vec![10, 1, 1, 6];
        let output = vec![9, 0, 1, 6];
        let result = Solution::final_prices(prices);
        assert_eq!(result, output);
    }
}
