fn main() {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];

    // Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
    //
    let heights = vec![
        vec![
            19, 16, 16, 12, 14, 0, 17, 11, 2, 0, 18, 9, 13, 16, 8, 8, 8, 13, 17, 9, 16, 9, 4, 7, 1,
            19, 10, 7, 0, 15,
        ],
        vec![
            0, 11, 4, 14, 9, 0, 6, 13, 16, 5, 19, 9, 4, 5, 4, 12, 0, 13, 0, 7, 9, 12, 13, 15, 3, 7,
            4, 9, 15, 1,
        ],
        vec![
            13, 14, 12, 12, 12, 16, 6, 15, 13, 1, 8, 9, 11, 14, 14, 10, 19, 11, 10, 0, 5, 18, 4,
            12, 7, 13, 17, 15, 18, 1,
        ],
        vec![
            16, 14, 19, 5, 8, 2, 11, 17, 7, 1, 4, 6, 5, 18, 7, 15, 6, 19, 18, 12, 1, 14, 2, 2, 0,
            9, 15, 14, 13, 19,
        ],
        vec![
            17, 4, 12, 9, 12, 10, 12, 10, 4, 5, 12, 7, 2, 12, 18, 10, 10, 8, 6, 1, 5, 13, 10, 3, 5,
            3, 11, 4, 8, 11,
        ],
        vec![
            8, 19, 18, 9, 6, 2, 7, 3, 19, 6, 0, 17, 9, 12, 11, 1, 15, 11, 18, 1, 8, 11, 1, 11, 16,
            7, 8, 17, 15, 0,
        ],
        vec![
            7, 0, 5, 11, 1, 7, 12, 18, 12, 1, 5, 2, 11, 7, 18, 12, 0, 11, 9, 18, 5, 2, 3, 1, 1, 1,
            8, 14, 19, 5,
        ],
        vec![
            2, 14, 2, 16, 17, 19, 10, 16, 1, 16, 16, 3, 19, 12, 13, 17, 19, 12, 16, 10, 16, 8, 16,
            12, 6, 12, 13, 17, 9, 12,
        ],
        vec![
            8, 1, 10, 5, 7, 0, 15, 19, 8, 15, 4, 12, 18, 18, 13, 11, 5, 2, 8, 3, 15, 4, 3, 7, 7,
            14, 15, 11, 6, 16,
        ],
        vec![
            0, 5, 13, 19, 1, 1, 2, 4, 16, 2, 16, 9, 15, 15, 10, 10, 18, 11, 17, 1, 5, 14, 5, 19, 7,
            0, 13, 7, 13, 7,
        ],
        vec![
            11, 6, 16, 12, 4, 2, 9, 11, 17, 19, 12, 10, 6, 16, 17, 5, 1, 18, 19, 7, 15, 1, 14, 0,
            3, 19, 7, 3, 4, 13,
        ],
        vec![
            4, 11, 8, 10, 10, 19, 7, 18, 4, 2, 2, 14, 6, 9, 18, 14, 2, 16, 5, 3, 19, 17, 4, 3, 7,
            1, 12, 2, 4, 3,
        ],
        vec![
            14, 16, 3, 11, 13, 13, 6, 16, 18, 0, 17, 19, 4, 1, 14, 12, 4, 17, 5, 19, 8, 13, 15, 3,
            15, 4, 1, 14, 12, 10,
        ],
        vec![
            13, 2, 12, 2, 16, 12, 19, 10, 19, 12, 19, 14, 12, 17, 16, 3, 13, 7, 3, 15, 16, 7, 10,
            15, 14, 10, 6, 5, 2, 18,
        ],
    ];

    //     Output
    // [[0,13],[2,13],[2,14],[3,13]]
    // Expected
    // [[0,29],[1,28],[2,28],[12,0],[12,1],[13,0]]
    println!("{:?}", pacific_atlantic(heights));
}

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut chain: Vec<Vec<i32>> = Vec::new();
    for i in 0..heights.len() {
        for j in 0..heights[0].len() {
            let mut visited = vec![vec![false; heights[0].len()]; heights.len()];
            let old_cell = heights[i][j];
            let mut top_left = false;
            let mut bot_right = false;
            let old_i = i;
            let old_j = j;
            dfs(
                &heights,
                &mut visited,
                i as i32,
                j as i32,
                old_cell,
                &mut chain,
                &mut top_left,
                &mut bot_right,
                old_i as i32,
                old_j as i32,
                -1,
                -1,
            );
            // println!("i={} , j={} ", i, j);
            // println!("DEBUGPRINT[2]: main.rs:17: visited={:?}", visited);
            // println!("DEBUGPRINT[3]: main.rs:14: chain={:?}", chain);
        }
    }
    chain
}

fn is_valid(heights: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
    if i < 0 || i >= heights.len() as i32 {
        return false;
    }
    if j < 0 || j >= heights[0].len() as i32 {
        return false;
    }
    true
}

fn dfs(
    heights: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    i: i32,
    j: i32,
    old_cell: i32,
    chain: &mut Vec<Vec<i32>>,
    top_left: &mut bool,
    bot_right: &mut bool,
    old_i: i32,
    old_j: i32,
    par_i: i32,
    par_j: i32,
) {
    if par_i == -1 {
    } else if !is_valid(heights, i, j)
        || visited[i as usize][j as usize]
        || heights[i as usize][j as usize] > heights[par_i as usize][par_j as usize]
        || (*top_left && *bot_right)
    {
        return;
    }

    if j == 0 || i == 0 {
        *top_left = true;
    }

    if j == heights[0].len() as i32 - 1 || i == heights.len() as i32 - 1 {
        *bot_right = true;
    }

    if *top_left && *bot_right {
        chain.push(vec![old_i, old_j]);
        return;
    }

    visited[i as usize][j as usize] = true;

    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    for (add_i, add_j) in directions {
        let id = add_i + i;
        let jd = add_j + j;

        dfs(
            heights, visited, id, jd, old_cell, chain, top_left, bot_right, old_i, old_j, i, j,
        );
    }
}
