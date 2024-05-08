pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut score_with_indices = score
        .iter()
        .enumerate()
        .map(|(idx, &grade)| (grade, idx))
        .collect::<Vec<_>>();

    score_with_indices.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    let mut count = 0;
    let mut res = vec![String::new(); score.len()];

    for (_, idx) in score_with_indices {
        if count >= 3 {
            res[idx] = (count + 1).to_string();
        } else if count == 0 {
            res[idx] = "Gold Medal".to_string();
        } else if count == 1 {
            res[idx] = "Silver Medal".to_string();
        } else if count == 2 {
            res[idx] = "Bronze Medal".to_string();
        }
        count += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let score = vec![10, 3, 8, 9, 4];
        // 3, 4, 8, 9, 10
        // 10, 9, 8, 4, 3
        let output = vec![
            "Gold Medal".to_string(),
            "5".to_string(),
            "Bronze Medal".to_string(),
            "Silver Medal".to_string(),
            "4".to_string(),
        ];
        let result = find_relative_ranks(score);
        assert_eq!(result, output);
    }
}
