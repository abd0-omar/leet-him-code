pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for i in 0..land.len() {
        for j in 0..land[0].len() {
            if land[i][j] == 1 {
                let mut top_left = Vec::from([i as i32, j as i32]);
                let mut bot_right = Vec::from([i as i32, j as i32]);
                _find_farmland(&mut land, i, j, &mut bot_right, -1, -1);
                top_left.extend_from_slice(&bot_right);
                result.push(top_left);
            }
        }
    }
    result
}

pub fn _find_farmland(
    land: &mut Vec<Vec<i32>>,
    i: usize,
    j: usize,
    bot_right: &mut Vec<i32>,
    last_dir_i: i32,
    last_dir_j: i32,
) -> () {
    if !((0..land.len()).contains(&i) && (0..land[0].len()).contains(&j) && land[i][j] == 1) {
        return;
    }
    if (last_dir_i == 1 && last_dir_j == 0) || (last_dir_i == 0 && last_dir_j == 1) {
        bot_right[0] = i as i32;
        bot_right[1] = j as i32;
    }

    land[i][j] = 0;

    for (di, dj) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
        let ni = (di + i as i32) as usize;
        let nj = (dj + j as i32) as usize;

        _find_farmland(land, ni, nj, bot_right, di, dj);
    }
}

pub fn find_farmland_bfs(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut q = std::collections::VecDeque::new();

    let mut result: Vec<Vec<i32>> = Vec::new();
    for i in 0..land.len() {
        for j in 0..land[0].len() {
            if land[i][j] == 1 {
                // bfs
                q.push_back((i, j));
                let mut top_left = Vec::from([i as i32, j as i32]);
                let mut bot_right = Vec::from([i as i32, j as i32]);
                while !q.is_empty() {
                    let size = q.len();
                    for _ in 0..size {
                        let (cur_i, cur_j) = q.pop_front().unwrap();

                        // visited
                        land[cur_i][cur_j] = 0;
                        bot_right[0] = cur_i as i32;
                        bot_right[1] = cur_j as i32;

                        for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                            let ni = (di + cur_i as i32) as usize;
                            let nj = (dj + cur_j as i32) as usize;

                            if (0..land.len()).contains(&ni)
                                && (0..land[0].len()).contains(&nj)
                                && land[ni][nj] == 1
                            {
                                q.push_back((ni, nj));
                            }
                        }
                    }
                }
                //push new_vec to result
                top_left.extend_from_slice(&bot_right);
                result.push(top_left);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let land = vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]];
        let output = vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]];
        let result = find_farmland(land);
        assert_eq!(result, output);
    }
}
