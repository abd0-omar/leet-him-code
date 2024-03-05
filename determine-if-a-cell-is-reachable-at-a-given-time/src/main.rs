fn main() {
    println!("Hello, world!");
    let sx = 2;
    let sy = 4;
    let fx = 7;
    let fy = 7;
    // let fx = 2;
    // let fy = 5;
    let t = 7;
    println!("{}", is_reachable_at_time(sx, sy, fx, fy, t));
    let sx = 3;
    let sy = 1;
    let fx = 7;
    let fy = 3;
    let t = 3;
    // println!("{}", is_reachable_at_time(sx, sy, fx, fy, t));
}

pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
    let mut q = std::collections::VecDeque::new();
    q.push_back((sy, sx));
    let mut visited = std::collections::HashSet::new();
    // let mut visited = vec![vec![false]];
    visited.insert((sy, sx));
    let mut lvl = 0;
    if (sx, sy) == (fx, fy) {
        if t == 0 {
            return true;
        }
        if t == 1 {
            return false;
        }
        if t >= 2 {
            return true;
        }
    }
    while !q.is_empty() {
        lvl += 1;
        //println!("lvl={:?}", lvl);
        let size = q.len();
        for _ in 0..size {
            //println!("q={:?}", q);
            let (i, j) = q.pop_front().unwrap();
            if i != fy && j != fx {
                // move diagonally
                //println!("diagonal mode");
                for (di, dj) in &[(1, 1), (-1, -1), (1, -1), (-1, 1)] {
                    let i = i + di;
                    let j = j + dj;
                    if i != -1 && j != -1 && visited.get(&(i, j)).is_none() {
                        if (i, j) == (fy, fx) {
                            // println!("i={:?}", i);
                            // println!("j={:?}", j);
                            // println!("lvl={:?}", lvl);
                            // println!("t={:?}", t);
                            return t >= lvl;
                        }
                        visited.insert((i, j));
                        q.push_back((i, j));
                    }
                }
            } else {
                //println!("normal mode");
                for (di, dj) in &[(1, 0), (0, -1), (-1, 0), (0, 1)] {
                    let i = i + di;
                    let j = j + dj;
                    if i != -1 && j != -1 && visited.get(&(i, j)).is_none() {
                        if (i, j) == (fy, fx) {
                            //println!("i={:?}", i);
                            //println!("j={:?}", j);
                            // println!("lvl={:?}", lvl);
                            // println!("t={:?}", t);
                            return t >= lvl;
                        }
                        visited.insert((i, j));
                        q.push_back((i, j));
                    }
                }
            }
        }
    }
    false
}
