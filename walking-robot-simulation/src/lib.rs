use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let (mut i, mut j) = (0, 0);
    let mut direction = Direction::North;
    let mut hs = HashSet::new();

    for obstacle in obstacles {
        let (ob_i, ob_j) = (obstacle[0], obstacle[1]);
        hs.insert((ob_i, ob_j));
    }

    let mut result = 0;

    for command in commands {
        match command {
            -1 => {
                direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            }
            -2 => {
                direction = match direction {
                    Direction::North => Direction::West,
                    Direction::West => Direction::South,
                    Direction::South => Direction::East,
                    Direction::East => Direction::North,
                }
            }
            x => {
                for _ in 0..x {
                    match direction {
                        Direction::North => {
                            if !hs.contains(&(i, j + 1)) {
                                j += 1
                            }
                        }
                        Direction::East => {
                            if !hs.contains(&(i + 1, j)) {
                                i += 1
                            }
                        }
                        Direction::South => {
                            if !hs.contains(&(i, j - 1)) {
                                j -= 1
                            }
                        }
                        Direction::West => {
                            if !hs.contains(&(i - 1, j)) {
                                i -= 1
                            }
                        }
                    }
                }
                result = result.max(i.abs().pow(2) + j.abs().pow(2));
            }
        }
    }

    result
}

fn increase_j(i: i32, j: i32, x: i32, hs: &HashSet<(i32, i32)>) -> i32 {
    let mut j = j;
    for _ in 0..x {
        let nj = j + 1;
        if !hs.contains(&(i, nj)) {
            j = nj;
        }
    }
    j
}

fn decrease_j(i: i32, j: i32, x: i32, hs: &HashSet<(i32, i32)>) -> i32 {
    let mut j = j;
    for _ in 0..x {
        let nj = j - 1;
        if !hs.contains(&(i, nj)) {
            j = nj;
        }
    }
    j
}

fn increase_i(i: i32, j: i32, x: i32, hs: &HashSet<(i32, i32)>) -> i32 {
    let mut i = i;
    for _ in 0..x {
        let ni = i + 1;
        if !hs.contains(&(ni, j)) {
            i = ni;
        }
    }
    i
}

fn decrease_i(i: i32, j: i32, x: i32, hs: &HashSet<(i32, i32)>) -> i32 {
    let mut i = i;
    for _ in 0..x {
        let ni = i - 1;
        if !hs.contains(&(ni, j)) {
            i = ni;
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let commands = vec![4, -1, 3];
        let obstacles = vec![];
        let output = 25;
        let result = robot_sim(commands, obstacles);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let commands = vec![4, -1, 4, -2, 4];
        let obstacles = vec![vec![2, 4]];
        let output = 65;
        let result = robot_sim(commands, obstacles);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let commands = vec![6, -1, -1, 6];
        let obstacles = vec![];
        let output = 36;
        let result = robot_sim(commands, obstacles);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let commands = vec![-2, -1, -2, 3, 7];
        let obstacles = vec![
            vec![1, -3],
            vec![2, -3],
            vec![4, 0],
            vec![-2, 5],
            vec![-5, 2],
            vec![0, 0],
            vec![4, -4],
            vec![-2, -5],
            vec![-1, -2],
            vec![0, 2],
        ];
        let output = 100;
        let result = robot_sim(commands, obstacles);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let commands = vec![-2, -1, 8, 9, 6];
        let obstacles = vec![
            vec![-1, 3],
            vec![0, 1],
            vec![-1, 5],
            vec![-2, -4],
            vec![5, 4],
            vec![-2, -3],
            vec![5, -1],
            vec![1, -1],
            vec![5, 5],
            vec![5, 2],
        ];
        let output = 0;
        let result = robot_sim(commands, obstacles);
        assert_eq!(result, output);
    }
}
