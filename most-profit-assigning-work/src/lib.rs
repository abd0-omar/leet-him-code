// binary search
pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let n = difficulty.len();
    let mut diff_prof = Vec::with_capacity(n);
    for i in 0..n {
        diff_prof.push((difficulty[i], profit[i]));
    }
    diff_prof.sort_unstable();
    dbg!(&diff_prof);
    // prefix_vec of max_profit till now
    for i in 1..n {
        diff_prof[i].1 = diff_prof[i].1.max(diff_prof[i - 1].1);
    }
    dbg!(&diff_prof);

    let mut count = 0;
    for one_worker in worker {
        let mut l = 0;
        let mut r = n as i32 - 1;
        let mut ans = None;
        while l <= r {
            let mid = l + (r - l) / 2;
            // T T T F F F
            // we want last true, and it's guaranteed to have the max_profit uptille now becuase we did prefix max_profit till now
            if diff_prof[mid as usize].0 <= one_worker {
                l = mid + 1;
                ans = Some(diff_prof[mid as usize].1);
            } else {
                r = mid - 1;
            }
        }
        if let Some(a) = ans {
            count += a;
        }
    }

    count
}

// two pointers
pub fn max_profit_assignment1(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
    let (n, m) = (difficulty.len(), worker.len());
    worker.sort_unstable();

    // sort by position, nice trick
    // could also make a tuple of diff and worker and sort on that but sort by pos is nice
    let mut pos = (0..n).collect::<Vec<usize>>();

    pos.sort_unstable_by(|&a, &b| difficulty[a].cmp(&difficulty[b]));
    println!("DEBUGPRINT[1]: main.rs:16: pos={:#?}", pos);

    let mut i = 0;
    let mut j = 0;
    let mut max_prof = 0;
    let mut sum = 0;
    while i < m {
        // let mut j = 0;
        while j < n && worker[i] >= difficulty[pos[j]] {
            max_prof = std::cmp::max(profit[pos[j]], max_prof);
            j += 1;
        }
        sum += max_prof;
        i += 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let difficulty = vec![2, 4, 6, 8, 10];
        let profit = vec![10, 20, 30, 40, 50];
        let worker = vec![4, 5, 6, 7];
        let output = 100;
        let result = max_profit_assignment(difficulty, profit, worker);
        assert_eq!(result, output);
    }
}
