pub fn num_teams(rating: Vec<i32>) -> i32 {
    // the input is low so we can brute force it with three loops
    // will do the three loops two times
    // one time checkin for lower
    // one time checkin for higher
    count_teams(&rating, false) + count_teams(&rating, true)
}

fn count_teams(rating: &[i32], is_lower: bool) -> i32 {
    let mut count = 0;

    let n = rating.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if is_lower {
                    if rating[i] < rating[j] && rating[j] < rating[k] {
                        count += 1;
                    }
                } else {
                    if rating[i] > rating[j] && rating[j] > rating[k] {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rating = vec![2, 5, 3, 4, 1];
        let output = 3;
        // Explanation: We can form three teams given the conditions. (2,3,4), (5,4,1), (5,3,1).
        let result = num_teams(rating);
        assert_eq!(result, output);
    }
}
