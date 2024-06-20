pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    position.sort_unstable();
    // let's assume that the answer is 3, that's binary search!
    // let mut l = 0;
    // let mut r = position.len() as i32 - 1;
    // while l <= r {
    //     // potential answer
    //     let mid = l + (r - l) / 2;
    //     // 1 2 3 4 5 6 7
    //     // T F T F F F F
    //     // reverse loop till we find the answer?
    // }
    // let n = position.len();
    // // the max num is 7, so we start from 6
    // for i in (1..position[n - 1] - 1).rev() {
    //     // find first true (T)
    //     if possible(&position, m, i as usize) {
    //         return i;
    //     }
    // }
    // back to binary search
    // that was wrong
    //     //     // 1 2 3 4 5 6 7
    //    // T F T F F F F
    // it was supposed to be like this
    //     //     // 1 2 3 4 5 6 7
    //    // T T T F F F F
    // 2 will result to 1 in the end but it will work
    // so find the last true (T)
    let n = position.len();
    let mut l = 1;
    let mut r = position[n - 1] - 1;
    let mut ans = -1;
    while l <= r {
        let mid = l + (r - l) / 2;
        dbg!(mid);
        if possible(&position, m, mid) {
            l = mid + 1;
            ans = mid;
        } else {
            r = mid - 1;
        }
    }

    ans
}

fn possible(position: &[i32], m: i32, mid: i32) -> bool {
    let mut count = 1;
    // take the first pos
    let mut last_position = position[0];

    for &pos in position.iter().skip(1) {
        if pos - last_position >= mid {
            count += 1;
            last_position = pos;
            if count == m {
                return true;
            }
        }
    }

    false
}

// fn possible(position: &[i32], m: i32, i: usize) -> bool {

//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let position = vec![1, 2, 3, 4, 7];
        let m = 3;
        let output = 3;
        let result = max_distance(position, m);
        assert_eq!(result, output);
    }
}
