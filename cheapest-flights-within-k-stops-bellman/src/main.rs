fn main() {
    println!("Hello, world!");
}

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;
    let mut dist = vec![i32::MAX; n];
    dist[src as usize] = 0;

    for _ in 0..k + 1 {
        let mut next_dist = dist.clone();
        let mut updated = false;
        for flight in &flights {
            let from = flight[0] as usize;
            let to = flight[1] as usize;
            let cost = flight[2];
            if next_dist[from] != i32::MAX && dist[from] + cost < dist[to] {
                next_dist[to] = dist[from] + cost;
                updated = true;
            }
        }
        dist = next_dist;
        if !updated {
            break;
        }
    }

    if dist[dst as usize] == i32::MAX {
        -1 // Destination is unreachable
    } else {
        dist[dst as usize]
    }
}
