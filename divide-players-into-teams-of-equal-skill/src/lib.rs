pub fn divide_players(mut skill: Vec<i32>) -> i64 {
    skill.sort_unstable();
    let n = skill.len();
    let first = skill[0];
    let last = skill[n - 1];
    let total = first + last;
    let mut result = (first * last) as i64;
    dbg!(first);
    dbg!(last);
    for i in 1..n / 2 {
        let first = skill[i];
        let last = skill[n - i - 1];
        dbg!(first);
        dbg!(last);
        if total != first + last {
            return -1;
        }
        result += (first * last) as i64;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let skill = vec![3, 2, 5, 1, 3, 4];
        let output = 22;
        let result = divide_players(skill);
        assert_eq!(result, output);
    }
}
