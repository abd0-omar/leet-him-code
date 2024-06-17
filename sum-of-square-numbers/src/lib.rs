pub fn judge_square_sum(c: i32) -> bool {
    // 1, 2, 3, 4, 5
    // i  j
    // i * i + j * j == 5
    // brute force

    // for i in 0..=c {
    //     for j in i..=c {
    //         if i * i + j * j == c {
    //             dbg!(i * i + j * j);
    //             return true;
    //         }
    //     }
    // }
    // false
    // TLE
    // binary search
    // maybe two poitners if this won't work
    // it didn't work for O(nlogn)
    // let mut i = 0;
    // while i * i <= c {
    //     let mut l = i;
    //     let mut r = (c as f64).sqrt() as i32;
    //     while l <= r {
    //         let mid = l + (r - l) / 2;
    //         let mult = mid * mid + i * i;
    //         if mult == c {
    //             return true;
    //         } else if mult < c {
    //             l = mid + 1;
    //         } else {
    //             r = mid - 1;
    //         }
    //     }
    //     i += 1;
    // }
    // false
    // double binary search maybe
    // let mut l = 0;
    // let mut r = c;
    // while l <= r {
    //     let mid = l - (r - l) / 2;
    //     match possible(mid, c) {
    //         Position::Correct => return true,
    //         Position::TooBig => r = mid - 1,
    //         Position::TooSmall => l = mid + 1,
    //     }
    // }
    // false
    // yeah that was a dumb idea, but I like it because it used enums
    // // two pointers (my last hope)
    let mut l = 0;
    let mut r = (c as f64).sqrt() as i32;

    while l <= r {
        let sum = (l as i64 * l as i64) + (r as i64 * r as i64);

        if sum == c as i64 {
            return true;
        } else if sum < c as i64 {
            l += 1;
        } else {
            r -= 1;
        }
    }

    false
}

// pub enum Position {
//     Correct,
//     TooBig,
//     TooSmall,
// }

// fn possible(old_mid: i32, c: i32) -> Position {
//     let mut l = old_mid;
//     let mut r = c;
//     let mut ans = -1;
//     while l <= r {
//         let mid = l + (r - l) / 2;
//         let mult = old_mid * old_mid + mid * mid;
//         if mult == c {
//             return Position::Correct;
//         } else if mult < c {
//             l = mid + 1;
//             ans = mult;
//         } else {
//             r = mid - 1;
//             ans = mult;
//         }
//     }
//     if ans < c {
//         Position::TooSmall
//     } else {
//         Position::TooBig
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let c = 5;
        let output = true;
        // Explanation: 1 * 1 + 2 * 2 = 5
        let result = judge_square_sum(c);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let c = 2;
        let output = true;
        // 1*1 + 1*1 == 2
        let result = judge_square_sum(c);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let c = 1;
        let output = true;
        // 0 * 0 + 1 * 1 == 1
        let result = judge_square_sum(c);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let c = 3;
        let output = false;
        let result = judge_square_sum(c);
        assert_eq!(result, output);
    }
}
