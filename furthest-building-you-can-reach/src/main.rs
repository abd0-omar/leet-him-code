fn main() {
    println!("Hello, world!");
    let heights = vec![4, 2, 7, 6, 9, 14, 12];
    // [0, 0, 5, 0, 3, 5, 0]
    // [0, 1, 3, 1, 0, 20, 0]
    let bricks = 5;
    let ladders = 1;
    println!("{}", furthest_building(heights, bricks, ladders));
    // Output: 4
    // Explanation: Starting at building 0, you can follow these steps:
    // - Go to building 1 without using ladders nor bricks since 4 >= 2.
    // - Go to building 2 using 5 bricks. You must use either bricks or ladders because 2 < 7.
    // - Go to building 3 without using ladders nor bricks since 7 >= 6.
    // - Go to building 4 using your only ladder. You must use either bricks or ladders because 6 < 9.
    // It is impossible to go beyond building 4 because you do not have any more bricks or ladders.
    // println!("{}", furthest_building(heights, bricks, ladders));

    let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
    let bricks = 10;
    let ladders = 2;
    println!("{}", furthest_building(heights, bricks, ladders));
    // Output: 7
    let heights = vec![14, 3, 19, 3];
    let bricks = 17;
    let ladders = 0;
    println!("{}", furthest_building(heights, bricks, ladders));
}

#[derive(Clone, Debug)]
#[allow(unused)]
enum Tyeb {
    Brickz(i32),
    Ladderz(i32),
    Holderz,
}

pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut bh = BinaryHeap::new();

    let n = heights.len();
    let mut ladderz = ladders;
    let mut i = 1;

    while i < n && ladderz > 0 {
        if heights[i] > heights[i - 1] {
            bh.push(Reverse(heights[i] - heights[i - 1]));
            ladderz -= 1;
        }
        i += 1;
    }

    println!("bh={:?}", bh);

    while i < n {
        if heights[i] > heights[i - 1] {
            // bh.push(heights[i] - heights[i - 1]);
            let diff = heights[i] - heights[i - 1];
            if let Some(p) = bh.peek() {
                let p = p.0;
                if diff > p {
                    let popped = bh.pop().unwrap().0;
                    bricks -= popped;
                    if bricks < 0 {
                        // i -= 1;
                        break;
                    }
                    bh.push(Reverse(diff));
                } else {
                    bricks -= diff;
                    if bricks < 0 {
                        break;
                    }
                }
            } else {
                // println!("{}", i);
                bricks -= diff;
                if bricks < 0 {
                    // i -= 1;
                    break;
                }
            }
        }
        i += 1;
    }

    i as i32 - 1
}
