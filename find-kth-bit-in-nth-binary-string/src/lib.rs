pub fn find_kth_bit(n: i32, k: i32) -> char {
    // clearly it's a recursion problem
    // but not a problem for me
    // and I see no way of memoization
    // so it's a brute-force recursion with no tricks
    let result = gen_sets(n - 1);
    // dbg!(&result);
    result[k as usize - 1]
}

fn gen_sets(n: i32) -> Vec<char> {
    if n == 0 {
        return vec!['0'];
    }
    let prev = gen_sets(n - 1);
    // prev + '1' + reverse(invert(prev))
    let mut prev_invert = Vec::new();
    for &letter in &prev {
        prev_invert.push(if letter == '1' { '0' } else { '1' })
    }
    prev_invert.reverse();
    // let prev_invert_reverse = prev_invert;
    let mut cur_set = Vec::with_capacity(prev.len() + 1 + prev_invert.len());
    cur_set.extend_from_slice(&prev);
    cur_set.push('1');
    cur_set.extend_from_slice(&prev_invert);
    // dbg!(&cur_set);
    return cur_set;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 3;
        let k = 1;
        let output = '0';
        let result = find_kth_bit(n, k);
        assert_eq!(result, output);
    }

    #[test]
    fn zero_case() {
        let n = 1;
        let k = 1;
        let output = '0';
        let result = find_kth_bit(n, k);
        assert_eq!(result, output);
    }
}
