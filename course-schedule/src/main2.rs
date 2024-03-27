use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0], vec![0, 3], vec![3, 2]];
    println!("{}", can_finish(num_courses, prerequisites));
    //     Example 1:
    //
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    println!("{}", can_finish(num_courses, prerequisites));
    // Output: true
    // Explanation: There are a total of 2 courses to take.
    // To take course 1 you should have finished course 0. So it is possible.
    // Example 2:
    //
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    println!("{}", can_finish(num_courses, prerequisites));
    // Output: false
    let num_courses = 5;
    let prerequisites = vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]];
    println!("{}", can_finish(num_courses, prerequisites));
    // Output: true
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    if prerequisites.len() == 0 {
        return num_courses == prerequisites.len() as i32;
    }
    // let mut graph: Vec<Vec<i32>> = vec![vec![]; prerequisites.len()];
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..prerequisites.len() {
        let from = prerequisites[i][0];
        let to = prerequisites[i][1];
        // graph[from as usize].push(to);
        graph.entry(from).or_insert(vec![]).push(to);
    }

    println!("DEBUGPRINT[1]: main.rs:10: graph={:?}", graph);

    let mut big_count = 0;
    let mut visited = HashSet::new();

    let mut ok = false;
    for i in 0..prerequisites.len() {
        let current = prerequisites[i][0];
        let mut count = 0;
        let mut validation = true;
        if visited.get(&current).is_none() {
            dfs(&graph, &mut visited, current, &mut count, &mut validation);
        }

        if !validation {
            return false;
        }

        if validation {
            big_count += count;
        }
        ok = validation;
        println!("DEBUGPRINT[3]: main.rs:46: count={:?}", count);
        println!("DEBUGPRINT[4]: main.rs:35: big_count={:?}", big_count);
        println!("DEBUGPRINT[1]: main.rs:29: visited={:?}", visited);
    }

    println!("DEBUGPRINT[4]: main.rs:35: big_count={:?}", big_count);

    // visited.len() as i32 == num_courses

    // if ok {
    //     visited.len() as i32 == num_courses - 1
    // } else {
    //     false
    // }
    true

    // big_count == num_courses
}

fn dfs(
    graph: &HashMap<i32, Vec<i32>>,
    visited: &mut HashSet<i32>,
    current: i32,
    count: &mut i32,
    validation: &mut bool,
) {
    if visited.get(&current).is_some() {
        *validation = false;
        println!("DEBUGPRINT[4]: main.rs:68: current={:?}", current);
        println!("hello");
        return;
    }

    visited.insert(current);
    println!("DEBUGPRINT[6]: main.rs:96: visited={:?}", visited);

    println!("DEBUGPRINT[5]: main.rs:75: current={:?}", current);
    *count += 1;
    // let graph_vec = graph.get(&current).unwrap();
    if let Some(graph_vec) = graph.get(&current) {
        println!("DEBUGPRINT[2]: main.rs:37: graph_vec={:?}", graph_vec);

        println!("DEBUGPRINT[2]: main.rs:78: count={:?}", count);

        for &neighbor in graph_vec {
            println!("DEBUGPRINT[3]: main.rs:40: neighbor={:?}", neighbor);

            dfs(graph, visited, neighbor, count, validation);
        }
    } else {
        return;
    }
}
