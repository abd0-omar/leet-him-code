// https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        // first solution that comes to mind is bfs01, but I didn't take the time before to learn
        // bfs01, I just know it's concept, it's very similar to another problem that I don't
        // remember that is solvable with bfs01 and dijkstra, and I will use dijkstra again
        // it's solvable by dp method too
        //
        // edit update:
        // It's the same as https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/description/
        let n = grid.len();
        let m = grid[0].len();
        let mut pq = std::collections::BinaryHeap::new();
        pq.push(Node::new(0, 0, 0));
        let mut distance = vec![vec![i32::MAX; m]; n];
        distance[0][0] = 0;
        //          1         2       3       4
        let dir = [
            (0, 1, Direction::Right),
            (0, -1, Direction::Left),
            (1, 0, Direction::Down),
            (-1, 0, Direction::Up),
        ];
        while !pq.is_empty() {
            let size = pq.len();
            for _ in 0..size {
                let cur_node = pq.pop().unwrap();
                if cur_node.i == n as i32 - 1 && cur_node.j == m as i32 - 1 {
                    return distance[cur_node.i as usize][cur_node.j as usize];
                }
                for (di, dj, direction) in dir.iter() {
                    let ni = (di + cur_node.i) as usize;
                    let nj = (dj + cur_node.j) as usize;
                    if (0..n).contains(&ni) && (0..m).contains(&nj) {
                        // check if distance already in the neighbor new cell
                        // bigger than cur + neighbor

                        // first get weight of neighbor which is 1 if wrong dir else 0
                        // it's better to do "enumerate" and check if "idx + 1" == "grid[ni][nj]"
                        // thought about it but was comfortable doing Direction enum
                        let neighbor_weight = match direction {
                            Direction::Right => {
                                if grid[cur_node.i as usize][cur_node.j as usize] == 1 {
                                    0
                                } else {
                                    1
                                }
                            }
                            Direction::Left => {
                                if grid[cur_node.i as usize][cur_node.j as usize] == 2 {
                                    0
                                } else {
                                    1
                                }
                            }
                            Direction::Down => {
                                if grid[cur_node.i as usize][cur_node.j as usize] == 3 {
                                    0
                                } else {
                                    1
                                }
                            }
                            Direction::Up => {
                                if grid[cur_node.i as usize][cur_node.j as usize] == 4 {
                                    0
                                } else {
                                    1
                                }
                            }
                        };
                        let new_distance =
                            distance[cur_node.i as usize][cur_node.j as usize] + neighbor_weight;
                        if new_distance < distance[ni][nj] {
                            distance[ni][nj] = new_distance;
                            pq.push(Node::new(new_distance, ni as i32, nj as i32));
                        }
                    }
                }
            }
        }
        unreachable!()
    }
}

enum Direction {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Eq, PartialEq)]
struct Node {
    weight: i32,
    i: i32,
    j: i32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl Node {
    fn new(weight: i32, i: i32, j: i32) -> Self {
        Self { weight, i, j }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
        ];
        let output = 3;
        let result = Solution::min_cost(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]];
        let output = 0;
        let result = Solution::min_cost(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let grid = vec![vec![1, 2], vec![4, 3]];
        let output = 1;
        let result = Solution::min_cost(grid);
        assert_eq!(result, output);
    }
}
