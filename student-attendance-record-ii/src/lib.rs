const MOD: i32 = 1_000_000_007;

pub fn check_record(n: i32) -> i32 {
    let mut memo = vec![vec![vec![-1; 3]; 2]; 1_000_01];
    eligible_combinations(n as usize, 0, 0, &mut memo)
}

fn eligible_combinations(
    n: usize,
    total_absences: i32,
    consecutive_lates: i32,
    memo: &mut Vec<Vec<Vec<i32>>>,
) -> i32 {
    if total_absences >= 2 || consecutive_lates >= 3 {
        return 0;
    }
    if n == 0 {
        return 1;
    }

    if memo[n][total_absences as usize][consecutive_lates as usize] != -1 {
        return memo[n][total_absences as usize][consecutive_lates as usize];
    }

    let mut count;
    count = eligible_combinations(n - 1, total_absences, 0, memo) % MOD;
    count = (count + eligible_combinations(n - 1, total_absences + 1, 0, memo)) % MOD;
    count =
        (count + eligible_combinations(n - 1, total_absences, consecutive_lates + 1, memo)) % MOD;

    memo[n][total_absences as usize][consecutive_lates as usize] = count;
    count
}

// pub fn _check_record(
//     n: usize,
//     status: &[char],
//     count: &mut i32,
//     freq: &mut HashMap<char, i32>,
//     idx: usize,
// ) -> () {
//     if idx == n {
//         *count += 1;
//         return;
//     }
//
//     for end in 0..status.len() {
//         if (status[end] == 'A' && freq.get(&status[end]).unwrap() >= &1)
//             || (status[end] == 'L' && freq.get(&status[end]).unwrap() >= &2)
//         {
//             continue;
//         }
//
//         *freq.entry(status[end]).or_insert(0) += 1;
//         _check_record(n, status, count, freq, idx + 1);
//         *freq.get_mut(&status[end]).unwrap() -= 1;
//     }
// }

// pub fn _check_record(
//     n: usize,
//     status: &[char],
//     mut cur_subset: Vec<char>,
//     total_subsets: &mut Vec<Vec<char>>,
//     idx: usize,
// ) -> () {
//     if cur_subset.len() == n || idx == n {
//         total_subsets.push(cur_subset);
//         return;
//     }
//
//     // check ur coin solu.
//     // pick and stay at your place
//     cur_subset.push(status[idx]);
//     let _pick = _check_record(n, status, cur_subset.clone(), total_subsets, idx + 1);
//     cur_subset.pop();
//
//     let _leave = _check_record(n, status, cur_subset, total_subsets, idx + 1);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 2;
        let output = 8;
        // Absent, Late, Present
        // not eligble
        // A >= 2
        // or
        // L >= 3
        // Explanation: There are 8 records with length 2 that are eligible for an award:
        // "PP", "AP", "PA", "LP", "PL", "AL", "LA", "LL"
        // Only "AA" is not eligible because there are 2 absences (there need to be fewer than 2).
        let result = check_record(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 1;
        let output = 3;
        let result = check_record(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let n = 10101;
        let output = 183236316;
        let result = check_record(n);
        assert_eq!(result, output);
    }
}
