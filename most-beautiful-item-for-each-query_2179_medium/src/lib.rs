// https://leetcode.com/problems/most-beautiful-item-for-each-query/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable();

        // maintain highest beauty
        let n = items.len();
        for i in 0..n - 1 {
            if items[i][1] > items[i + 1][1] {
                items[i + 1][1] = items[i][1];
            }
        }

        dbg!(&items);
        // queries = [price j, price j+1, ..]
        // less than or equal the query
        // binary search find the first price <= me, the answer is it's beauty
        let mut result = Vec::with_capacity(queries.len());
        for q in queries {
            let mut l = 0;
            let mut r = n - 1;
            let mut ans = 0;
            while l <= r {
                let mid = l + (r - l) / 2;
                if items[mid][0] <= q {
                    // we did it like this for duplicates too
                    ans = items[mid][1];
                    l = mid + 1;
                } else {
                    if let (_, true) = mid.overflowing_sub(1) {
                        break;
                    }
                    r = mid - 1;
                }
            }
            result.push(ans);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let items = vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]];
        let queries = vec![1, 2, 3, 4, 5, 6];
        let output = vec![2, 4, 5, 5, 6, 6];
        let result = Solution::maximum_beauty(items, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let items = vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]];
        let queries = vec![1];
        let output = vec![4];
        let result = Solution::maximum_beauty(items, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let items = vec![vec![10, 1000]];
        let queries = vec![5];
        let output = vec![0];
        let result = Solution::maximum_beauty(items, queries);
        assert_eq!(result, output);
    }
}
