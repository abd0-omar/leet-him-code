pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut max_elements = vec![(0, -1); 3];

    for (i, &s) in score.iter().enumerate() {
        if s > max_elements[0].0 {
            max_elements[0] = (s, i as i32);
            max_elements.sort_unstable();
            println!("max_elements={:?}", max_elements);
        }
    }

    let mut champs = vec![
        "Gold Medal".to_string(),
        "Silver Medal".to_string(),
        "Bronze Medal".to_string(),
    ]
    .into_iter();
    let mut score: Vec<String> = score.iter().map(|x| x.to_string()).collect();
    for (_, i) in max_elements.iter().rev() {
        if i >= &0 {
            if let Some(cha) = champs.next() {
                println!("cha={:?}", cha);
                score[*i as usize] = cha;
            }
        }
    }
    for s in score.clone() {
        if let Ok(s_parsed) = s.parse::<usize>() {
            println!("s_parsed={:?}", s_parsed);
            score[s_parsed] = s_parsed.to_string();
        }
    }
    score
    // 241
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let score = vec![10, 3, 8, 9, 4];
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
