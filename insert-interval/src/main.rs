fn main() {
    println!("Hello, world!");
    let intervals = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    let new_interval = vec![4, 8];
    // Output: [[1,2],[3,10],[12,16]]
    println!("{:?}", insert(intervals, new_interval));
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    println!("{:?}", insert(intervals, new_interval));
    // Output: [[1,5],[6,9]]
}
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::{max, min};
    let mut less = vec![];
    let mut more = vec![];
    let mut start = new_interval[0];
    let mut end = new_interval[1];

    for curr in intervals {
        if curr[1] < new_interval[0] {
            less.push(curr);
        } else if curr[0] > new_interval[1] {
            more.push(curr);
        } else {
            start = min(curr[0], start);
            end = max(curr[1], end);
        }
        println!("less={:?}", less);
        println!("more={:?}", more);
        println!("start={:?}", start);
        println!("end={:?}", end);
    }
    less.push(vec![start, end]);
    less.append(&mut more);
    less
}
