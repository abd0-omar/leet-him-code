pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let n = profits.len();
    let mut combined_cap_prof: Vec<(i32, i32)> = vec![(0, 0); n];
    for i in 0..n {
        combined_cap_prof[i].0 = capital[i];
        combined_cap_prof[i].1 = profits[i];
    }

    // sort by lowest capital
    combined_cap_prof.sort_unstable();
    let mut maximum_profits = std::collections::BinaryHeap::new();

    // we use global i to ignore the profits that we processed before
    let mut i = 0;
    for _ in 0..k {
        while i < n && combined_cap_prof[i].0 <= w {
            // push the all the profits within the range of w
            maximum_profits.push(combined_cap_prof[i].1);
            i += 1;
        }

        if let Some(highest_cur_prof) = maximum_profits.pop() {
            w += highest_cur_prof;
        } else {
            break;
        }
    }
    w
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let k = 2;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 1];
        let output = 4;
        let result = find_maximized_capital(k, w, profits, capital);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let k = 3;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 2];
        let output = 6;
        let result = find_maximized_capital(k, w, profits, capital);
        assert_eq!(result, output);
    }
}
