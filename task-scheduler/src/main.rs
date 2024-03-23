use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut freq: [i32; 26] = [0; 26];

    // Building frequency map
    for &ch in tasks.iter() {
        freq[(ch as u8 - b'A') as usize] += 1;
    }

    // Max heap to store frequencies
    let mut pq = BinaryHeap::new();
    for &f in freq.iter() {
        if f > 0 {
            pq.push(f);
        }
    }

    let mut time = 0;
    // Process tasks until the heap is empty
    while !pq.is_empty() {
        let mut cycle = n + 1;
        let mut store = VecDeque::new();
        let mut task_count = 0;
        // Execute tasks in each cycle
        while cycle > 0 && !pq.is_empty() {
            if let Some(top) = pq.pop() {
                if top > 1 {
                    store.push_back(top - 1);
                }
                task_count += 1;
                cycle -= 1;
            }
        }
        // Restore updated frequencies to the heap
        for x in store.into_iter() {
            pq.push(x);
        }
        // Add time for the completed cycle
        time += if pq.is_empty() { task_count } else { n + 1 };
    }
    time
}

fn main() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    let n = 2;
    println!("{}", least_interval(tasks, n)); // Output should be 8
}
