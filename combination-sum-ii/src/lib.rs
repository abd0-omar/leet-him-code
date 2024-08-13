pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort_unstable();
    let mut result = Vec::new();
    let mut cur_vec = Vec::new();
    back_track(&candidates, target, &mut cur_vec, 0, &mut result, 0);
    result
}

fn back_track(
    candidates: &[i32],
    target: i32,
    cur_vec: &mut Vec<i32>,
    cur_sum: i32,
    result: &mut Vec<Vec<i32>>,
    st: usize,
) {
    if cur_sum == target {
        result.push(cur_vec.clone());
        return;
    }
    if cur_sum > target {
        return;
    }

    for end in st..candidates.len() {
        // 1, 1, 2
        // we choose only the firest one
        // we choose the first one in the duplicates
        // if cur_idx (end) > st && have duplicates before it, then skip it
        if end > st && candidates[end] == candidates[end - 1] {
            continue;
        }
        cur_vec.push(candidates[end]);
        back_track(
            candidates,
            target,
            cur_vec,
            cur_sum + candidates[end],
            result,
            end + 1,
        );
        cur_vec.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let mut output = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        for vec in output.iter_mut() {
            vec.sort_unstable();
        }
        output.sort_unstable();
        let result = combination_sum2(candidates, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let candidates = vec![1];
        let target = 1;
        let mut output = vec![vec![1]];
        for vec in output.iter_mut() {
            vec.sort_unstable();
        }
        output.sort_unstable();
        let result = combination_sum2(candidates, target);
        assert_eq!(result, output);
    }
}
