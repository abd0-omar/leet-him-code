use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot.clone();
        let mut factory = factory.clone();

        robot.sort_unstable();
        factory.sort_unstable_by_key(|f| f[0]);

        let mut factory_positions = Vec::new();
        for f in factory {
            for _ in 0..f[1] {
                factory_positions.push(f[0]);
            }
        }

        // [[2, 2], [6, 2]]
        // flattened:
        // [2, 2, 6, 6]
        // dbg!(&factory_positions);

        let robot_count = robot.len();
        let factory_count = factory_positions.len();

        let mut memo = vec![vec![None; factory_count]; robot_count];

        Self::calculate_min_distance(0, 0, &robot, &factory_positions, &mut memo)
    }

    fn calculate_min_distance(
        robot_idx: usize,
        factory_idx: usize,
        robot: &[i32],
        factory_positions: &[i32],
        memo: &mut Vec<Vec<Option<i64>>>,
    ) -> i64 {
        if robot_idx == robot.len() {
            return 0;
        }

        if factory_idx == factory_positions.len() {
            return 1_000_000_000_000; // Equivalent to 1e12
        }

        if let Some(cached) = memo[robot_idx][factory_idx] {
            return cached;
        }

        let assign = (robot[robot_idx] - factory_positions[factory_idx]).abs() as i64
            + Self::calculate_min_distance(
                robot_idx + 1,
                factory_idx + 1,
                robot,
                factory_positions,
                memo,
            );

        let skip = Self::calculate_min_distance(
            robot_idx,
            factory_idx + 1,
            robot,
            factory_positions,
            memo,
        );

        let result = min(assign, skip);
        memo[robot_idx][factory_idx] = Some(result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let robot = vec![0, 4, 6];
        let factory = vec![vec![2, 2], vec![6, 2]];
        let output = 4;
        let result = Solution::minimum_total_distance(robot, factory);
        assert_eq!(result, output);
    }
}
