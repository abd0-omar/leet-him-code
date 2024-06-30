fn main() {
    use std::io::stdin;
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let m: usize = line.next().unwrap().parse().unwrap();
        let mut graph = vec![vec![0; m]; n];
        for i in 0..n {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            let mut line = line.split_whitespace();
            for j in 0..m {
                let input: usize = line.next().unwrap().parse().unwrap();
                graph[i][j] = input;
            }
        }
        // dbg!(&graph);
        for i in 0..n {
            for j in 0..m {
                let mut max_neighbor = 0;
                for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                    let ni = (i as i32 + di) as usize;
                    let nj = (j as i32 + dj) as usize;

                    // if valid && check if the cur_cell (i, j) bigger than neighbors
                    if (0..n).contains(&ni) && (0..m).contains(&nj) {
                        max_neighbor = max_neighbor.max(graph[ni][nj]);
                    }
                }
                if graph[i][j] > max_neighbor {
                    graph[i][j] = max_neighbor;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                print!("{} ", graph[i][j]);
            }
            println!();
        }
    }
}
