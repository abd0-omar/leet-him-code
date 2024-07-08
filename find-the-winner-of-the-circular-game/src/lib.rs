pub fn find_the_winner(n: i32, k: i32) -> i32 {
    // // put them in an array and kick them by k
    // // if the len of the array is 1 then you have a winner
    // let mut arr: Vec<i32> = (1..=n).collect();
    // let mut start = 0;
    // // 1, 2, 3, 4, 5
    // //st
    // //    st
    // // 1, 3, 4, 5
    // //    st
    // //       st
    // // 1, 3, 5            st = (st + 1) % arr.len() before deleting
    // //       st
    // // 1, 3, 5
    // while arr.len() > 1 {
    //     start -= 1;
    //     for _ in 0..k {
    //         start = (start + 1) % arr.len() as i32;
    //     }
    //     let delete_idx = start;
    //     println!("DEBUGPRINT[2]: lib.rs:19: delete_idx={:#?}", delete_idx);
    //     arr.remove(delete_idx as usize);
    //     // start = (start + 1) % arr.len() as i32;
    // }
    // // looking at the editorial after solving it
    // // the queue solution is nice
    let mut queue = std::collections::VecDeque::from_iter((1..=n).into_iter());
    // println!("DEBUGPRINT[3]: lib.rs:27: queue={:?}", queue);

    while queue.len() > 1 {
        // push the k - 1 friends to the end of the queue
        for _ in 0..k - 1 {
            queue.push_back(*queue.front().unwrap());
            queue.pop_front();
        }

        // pop the kth friend
        queue.pop_front();
    }

    queue[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 5;
        let k = 2;
        let output = 3;
        let result = find_the_winner(n, k);
        assert_eq!(result, output);
    }
}
