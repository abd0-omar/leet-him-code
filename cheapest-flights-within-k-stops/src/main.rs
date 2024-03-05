use std::cmp::Ordering;

#[allow(unused)]
fn main() {
    println!("Hello, world!");
    let n = 4;
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    let src = 0;
    let dst = 3;
    let k = 1;
    println!("{}", find_cheapest_price(n, flights, src, dst, k));

    let n = 3;
    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    let src = 0;
    let dst = 2;
    let k = 1;

    let n = 3;
    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    let src = 0;
    let dst = 2;
    let k = 0;

    let n = 4;
    let flights = vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]];
    let src = 0;
    let dst = 3;
    let k = 1;

    let n = 10;
    let flights = vec![
        vec![3, 4, 4],
        vec![2, 5, 6],
        vec![4, 7, 10],
        vec![9, 6, 5],
        vec![7, 4, 4],
        vec![6, 2, 10],
        vec![6, 8, 6],
        vec![7, 9, 4],
        vec![1, 5, 4],
        vec![1, 0, 4],
        vec![9, 7, 3],
        vec![7, 0, 5],
        vec![6, 5, 8],
        vec![1, 7, 6],
        vec![4, 0, 9],
        vec![5, 9, 1],
        vec![8, 7, 3],
        vec![1, 2, 6],
        vec![4, 1, 5],
        vec![5, 2, 4],
        vec![1, 9, 1],
        vec![7, 8, 10],
        vec![0, 4, 2],
        vec![7, 2, 8],
    ];

    let src = 6;
    let dst = 0;
    let k = 7;
}

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let flights_adj_list_new_and_improved = change_to_adj_list(n as usize, flights);
    dijkstra(
        &flights_adj_list_new_and_improved,
        src as usize,
        k as usize,
        dst,
        n as usize,
    )
}

fn change_to_adj_list(n: usize, flights: Vec<Vec<i32>>) -> Vec<Vec<Edge>> {
    let mut rez = vec![Vec::new(); n];
    for flight in flights.iter() {
        let from = flight[0] as usize;
        let to = flight[1] as usize;
        let w = flight[2] as usize;
        rez[from as usize].push(Edge {
            to,
            w,
            nodes_taken: 0,
        });
    }
    rez
}

#[derive(PartialEq, Eq, Ord, Clone, Debug)]
struct Edge {
    to: usize,
    w: usize,
    nodes_taken: usize,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // changing the heap to a min heap
        (other.w).partial_cmp(&(self.w))
    }
}

fn dijkstra(flights: &Vec<Vec<Edge>>, src: usize, k: usize, dst: i32, n: usize) -> i32 {
    use std::collections::BinaryHeap;
    let mut bh = BinaryHeap::new();

    let mut dist = vec![vec![usize::MAX; k + 1]; n];

    bh.push(Edge {
        to: src,
        w: 0,
        nodes_taken: 0,
    });

    //    src      nodes taken
    dist[src][0] = 0;

    while let Some(popped) = bh.pop() {
        let (curr_node, curr_weight, curr_count) = (popped.to, popped.w, popped.nodes_taken);

        println!("curr_count={:?}", curr_count);
        if curr_count > k && curr_node != dst as usize {
            continue;
        }

        if curr_node == dst as usize {
            return curr_weight as i32;
        }

        println!("dist={:?}", dist);
        if curr_count > 0 {
            if curr_weight > dist[curr_node][curr_count - 1] {
                println!("really {}", curr_count);
                continue;
            }
        }

        for neighbor in flights[curr_node as usize].iter() {
            let (neighbor_node, neighbor_weight, neighbor_count) =
                (neighbor.to, neighbor.w, neighbor.nodes_taken);

            let new_weight = curr_weight + neighbor_weight;
            let new_count = curr_count + 1;

            // if let Some(old_d) = dist[neighbor_node as usize][curr_count] {
            //     if old_d > dist[popped.to as usize][curr_count].unwrap() + neighbor_weight {
            //         dist[neighbor_node as usize][neighbor_count] = Some(new_weight);
            //     }
            // } else {
            //     dist[neighbor_node as usize][neighbor_count] = Some(new_weight);
            // }
            if dist[popped.to as usize][curr_count] == usize::MAX
                || dist[neighbor_node as usize][curr_count]
                    > dist[popped.to as usize][curr_count] + neighbor_weight
            {
                dist[neighbor_node as usize][neighbor_count] = new_weight;
            }

            bh.push(Edge {
                to: neighbor_node,
                w: new_weight,
                nodes_taken: new_count,
            })
        }
    }

    -1
}
