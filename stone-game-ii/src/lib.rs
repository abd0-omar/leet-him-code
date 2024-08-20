pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    /*
    Explanation:  If Alice takes one pile at the beginning, Bob takes two piles,
    then Alice takes 2 piles again. Alice can get 2 + 4 + 4 = 10 piles in total.
    If Alice takes two piles at the beginning, then Bob can take all three piles left.
    In this case, Alice get 2 + 7 = 9 piles in total. So we return 10 since it's larger.
    */
    // 2       A
    // 7, 9    B
    // 4, 4    A
    //  A    B
    // 10,  16
    //
    // 2, 7      A
    // 9, 4, 4   B
    //
    // return max score that Alice can achieve

    let mut memory = std::collections::HashMap::new();

    _stone_game(&piles, true, 0, 1, &mut memory)
}

fn _stone_game(
    piles: &[i32],
    is_alice: bool,
    idx: usize,
    m: usize,
    memory: &mut std::collections::HashMap<(usize, usize, bool), i32>,
) -> i32 {
    if idx == piles.len() {
        return 0;
    }

    if let Some(&ret) = memory.get(&(idx, m, is_alice)) {
        return ret;
    }

    let mut res = if is_alice { 0 } else { i32::MAX };
    let mut total = 0;

    for x in 1..2 * m + 1 {
        if idx + x - 1 == piles.len() {
            break;
        }

        total += piles[idx + x - 1];

        if is_alice {
            res = res.max(total + _stone_game(piles, false, idx + x, m.max(x), memory))
        } else {
            res = res.min(_stone_game(piles, true, idx + x, m.max(x), memory))
        };
    }

    memory.insert((idx, m, is_alice), res);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let piles = vec![2, 7, 9, 4, 4];
        let output = 10;
        let result = stone_game_ii(piles);
        assert_eq!(result, output);
    }
}
