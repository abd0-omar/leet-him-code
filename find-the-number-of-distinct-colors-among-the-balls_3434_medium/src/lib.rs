// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // it was just annoying not doing anything with `limit`
        use std::collections::HashMap;
        let mut balls_to_colors: HashMap<i32, i32> = HashMap::new();
        let mut colors_to_count: HashMap<i32, i32> = HashMap::new();

        let mut result = Vec::with_capacity(queries.len());
        let mut count_unique = 0;

        for query in queries {
            let ball = query[0];
            let color = query[1];
            let old_color = balls_to_colors.entry(ball);
            match old_color {
                std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                    let old_color = occupied_entry.get();
                    let old_color_count = colors_to_count.get_mut(old_color).unwrap();
                    if *old_color_count == 1 {
                        count_unique -= 1;
                    }
                    *old_color_count -= 1;

                    // insert new color
                    let cur_color_count = colors_to_count.entry(color).or_insert(0);
                    if *cur_color_count == 0 {
                        count_unique += 1;
                    }
                    *cur_color_count += 1;

                    balls_to_colors.insert(ball, color);
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(color);
                    let color_count = colors_to_count.entry(color).or_insert(0);
                    if *color_count == 0 {
                        count_unique += 1;
                    }
                    *color_count += 1;
                }
            }

            result.push(count_unique);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let limit = 4;
        let queries = vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]];
        let output = vec![1, 2, 2, 3];
        let result = Solution::query_results(limit, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let limit = 4;
        let queries = vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]];
        let output = vec![1, 2, 2, 3, 4];
        let result = Solution::query_results(limit, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let limit = 1;
        let queries = vec![vec![0, 1], vec![0, 4], vec![1, 2], vec![1, 5], vec![1, 4]];
        let output = vec![1, 1, 2, 2, 1];
        let result = Solution::query_results(limit, queries);
        assert_eq!(result, output);
    }
}
