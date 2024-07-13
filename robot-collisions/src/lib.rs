pub fn survived_robots_healths(
    positions: Vec<i32>,
    healths: Vec<i32>,
    directions: String,
) -> Vec<i32> {
    /*
        n robotots 1 indexed-> 1, 2, 3, 4
        positions 0 indexed -> 3, 5, 2, 6
        0, 1, 2, 3, 4, 5, 6
    pos:      3  1     2  4
    dir:      >  <     >  <
        if r l and health equal both go
        if r l and r > l, l goes out and r health decreases by 1 and continue in the same dir
        so a stack problem
        if stack.top != cur_dir
        then 3 options
        if stack.top health == cur_dir health
        just pop from the stack
        if stack.top health > cur_dir health
        just decrease stack.top health by 1
        if stack.top health < cur_dir health
        stack.pop and decrease cur_dir health by 1
        else (stack.top == cur_dir)
        push to the stack aka do nothing
        oh I forgot about the positions
        if they are not after each other in positions so they won't collide with each other
        but will collide with another robot
        positions 0 indexed -> 2, 3, 5, 6
        and sort dir also based on that
        so make a tuple of pos and dir
        positions 0 indexed -> (3, R), (5, L), (2, R), (6, L)
        sort based on the actual indices
        sorted              -> (2, R), (3, R), (5, L),  (6, L)
        ignore liness 9, 10, 11
    */
    let directions: Vec<char> = directions.chars().collect();
    let n = positions.len();
    let mut pos_dir_health = Vec::with_capacity(n);
    for i in 0..n {
        let pos = positions[i];
        let dir = directions[i];
        let health = healths[i];
        let idx = i;
        pos_dir_health.push((pos, dir, health, idx));
    }
    pos_dir_health.sort_unstable();

    // let mut stack: Vec<(i32, char, i32, usize)> = vec![];
    // for (idx, cur_all_stats) in pos_dir_health.iter_mut().enumerate() {
    //     if let Some(top) = stack.last_mut() {
    //         // if top.1 != cur_all_stats.1 {}
    //         if top.1 == 'R' && cur_all_stats.1 == 'L' {
    //             if top.2 == cur_all_stats.2 {
    //                 stack.pop();
    //             } else if top.2 > cur_all_stats.2 {
    //                 // decrease health by 1 and don't push the cur_all_stats
    //                 top.2 -= 1;
    //             } else {
    //                 // cur_all_stats.2 health > top.2 health
    //                 // pop the top in a while loop because there might be another robot behind me
    //                 // decrease cur_all_stats.2 health and push it in the end after the while loop
    //                 let mut dont_push_all_stats = false;
    //                 while !stack.is_empty() && cur_all_stats.2 > stack.last().unwrap().2 {
    //                     cur_all_stats.2 -= 1;
    //                     stack.pop();
    //                 }
    //                 if !stack.is_empty() && cur_all_stats.2 == stack.last().unwrap().2 {
    //                     // don't push cur_all_stats
    //                     dont_push_all_stats = true;
    //                     stack.pop();
    //                 } else if !stack.is_empty() && cur_all_stats.2 < stack.last().unwrap().2 {
    //                     dont_push_all_stats = true;
    //                     stack.last_mut().unwrap().2 -= 1;
    //                 }
    //                 if !dont_push_all_stats {
    //                     let new_all_stats = (
    //                         cur_all_stats.0,
    //                         cur_all_stats.1,
    //                         cur_all_stats.2,
    //                         cur_all_stats.3,
    //                     );
    //                     stack.push(new_all_stats);
    //                 }
    //             }
    //         } else {
    //             let new_all_stats = (
    //                 cur_all_stats.0,
    //                 cur_all_stats.1,
    //                 cur_all_stats.2,
    //                 cur_all_stats.3,
    //             );
    //             stack.push(new_all_stats);
    //         }
    //     } else {
    //         let new_all_stats = (
    //             cur_all_stats.0,
    //             cur_all_stats.1,
    //             cur_all_stats.2,
    //             cur_all_stats.3,
    //         );
    //         stack.push(new_all_stats);
    //     }
    //     dbg!(&stack);
    // }
    // simpler way to do it & also it didn't work for some reason

    let mut stack: Vec<(i32, char, i32, usize)> = vec![];
    for cur_all_stats in pos_dir_health.iter_mut() {
        while let Some(top) = stack.last_mut() {
            if top.1 == 'R' && cur_all_stats.1 == 'L' {
                if top.2 == cur_all_stats.2 {
                    stack.pop();
                    cur_all_stats.2 = 0;
                } else if top.2 > cur_all_stats.2 {
                    top.2 -= 1;
                    cur_all_stats.2 = 0;
                } else {
                    cur_all_stats.2 -= 1;
                    stack.pop();
                }
                if cur_all_stats.2 == 0 {
                    break;
                }
            } else {
                break;
            }
        }
        if cur_all_stats.2 > 0 {
            // instead of the dont_push_all_stats variable way
            // just remember to put a while inside the for loop not an if
            // it would save you so much time and would be more straightforward
            stack.push(*cur_all_stats);
        }
    }
    dbg!(&stack);
    let mut result = stack.into_iter().collect::<Vec<_>>();
    result.sort_by_key(|k| k.3);
    let result = result.into_iter().map(|(_, _, health, _)| health).collect();
    dbg!(&result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let positions = vec![3, 5, 2, 6];
        let healths = vec![10, 10, 15, 12];
        let directions = "RLRL".to_string();
        let output = vec![14];
        let result = survived_robots_healths(positions, healths, directions);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let positions = vec![5, 4, 3, 2, 1];
        let healths = vec![2, 17, 9, 15, 10];
        let directions = "RRRRR".to_string();
        let output = vec![2, 17, 9, 15, 10];
        let result = survived_robots_healths(positions, healths, directions);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let positions = vec![1, 2, 5, 6];
        let healths = vec![10, 10, 11, 11];
        let directions = "RLRL".to_string();
        let output = vec![];
        let result = survived_robots_healths(positions, healths, directions);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let positions = vec![1, 40];
        let healths = vec![10, 11];
        let directions = "RL".to_string();
        let output = vec![10];
        let result = survived_robots_healths(positions, healths, directions);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let positions = vec![3, 40];
        let healths = vec![49, 11];
        let directions = "LL".to_string();
        let output = vec![49, 11];
        let result = survived_robots_healths(positions, healths, directions);
        assert_eq!(result, output);
    }
}
