fn main() {
    println!("Hello, world!");
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let dr = [-1, 0, 1, 0];
    let dc = [0, -1, 0, 1];
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' && !visited[i][j] {
                dfs(&grid, &mut visited, &dr, &dc, i as i32, j as i32);
                count += 1;
            }
        }
    }
    count
}

fn is_valid(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if i < 0 || i >= grid.len() as i32 {
        return false;
    }
    if j < 0 || j >= grid[0].len() as i32 {
        return false;
    }
    true
}

fn dfs(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    dr: &[i32; 4],
    dc: &[i32; 4],
    i: i32,
    j: i32,
) {
    if !is_valid(grid, i, j)
        || visited[i as usize][j as usize]
        || grid[i as usize][j as usize] == '0'
    {
        return;
    }

    visited[i as usize][j as usize] = true;

    //     let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    //
    // for (add_x, add_y) in directions {
    //     dfs(grid, x + add_x, y + add_y);
    // }

    // if (0..matrix.len() as i32).contains(&ni)
    //             && (0..matrix[0].len() as i32).contains(&nj) {}

    for d in 0..4 {
        let id = i + dr[d];
        let jd = j + dc[d];

        if is_valid(grid, id, jd) && !visited[id as usize][jd as usize] {
            dfs(grid, visited, dr, dc, id, jd);
        }
    }
}
