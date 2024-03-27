fn main() {
    // println!("Hello, world!");
    // let num_courses = 2;
    // let prerequisites = vec![vec![1, 0], vec![0, 3], vec![3, 2]];
    // println!("{}", can_finish(num_courses, prerequisites));
    // //     Example 1:
    // //
    // let num_courses = 2;
    // let prerequisites = vec![vec![1, 0]];
    // println!("{}", can_finish(num_courses, prerequisites));
    // // Output: true
    // // Explanation: There are a total of 2 courses to take.
    // // To take course 1 you should have finished course 0. So it is possible.
    // // Example 2:
    // //
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    println!("{}", can_finish(num_courses, prerequisites));
    println!();
    // Output: false
    let num_courses = 5;
    let prerequisites = vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]];
    println!("{}", can_finish(num_courses, prerequisites));
    // Output: true
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Unvisited,
    Undecided,
    Processed,
    // todo
    // inprogress
    // done
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut graph: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
    for edge in &prerequisites {
        let from = edge[0] as usize;
        let to = edge[1];
        graph[from].push(to);
    }
    println!("DEBUGPRINT[1]: main.rs:34: graph={:?}", graph);

    let mut visited = vec![Status::Unvisited; num_courses as usize];
    println!("DEBUGPRINT[2]: main.rs:45: visited={:?}", visited);

    for i in 0..num_courses {
        let mut is_cycle = false;
        if visited[i as usize] == Status::Processed {
            continue;
        }
        dfs(&graph, &mut visited, i, &mut is_cycle);
        println!("DEBUGPRINT[4]: main.rs:50: visited={:?}", visited);
        if is_cycle {
            return false;
        }
    }

    true
}

fn dfs(graph: &Vec<Vec<i32>>, visited: &mut Vec<Status>, current: i32, is_cycle: &mut bool) {
    match visited[current as usize] {
        Status::Undecided => {
            *is_cycle = true;
            return;
        }
        Status::Processed => {
            return;
        }
        Status::Unvisited => {
            visited[current as usize] = Status::Undecided;
        }
    }
    for &next in &graph[current as usize] {
        dfs(graph, visited, next, is_cycle);
    }
    visited[current as usize] = Status::Processed;
}
