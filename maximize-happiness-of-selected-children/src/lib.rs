pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
    //253
    let mut happiness_sorted: Vec<_> = happiness.clone();
    happiness_sorted.sort_unstable_by(|a, b| b.cmp(a));
    // println!("happiness_sorted={:?}", happiness_sorted);
    let n = happiness.len();
    // maybe hashmap
    let mut visited = vec![false; n];

    let mut sum = 0;
    for _i in 0..k {
        for j in 0..n {
            if !visited[j] {
                for k in j + 1..n {
                    // println!("k={:?}", k);
                    if happiness_sorted[k] > 0 {
                        happiness_sorted[k] = happiness_sorted[k] - 1;
                    }
                    if happiness_sorted[k] == 0 {
                        break;
                    }
                }
                visited[j] = true;
                sum += happiness_sorted[j] as i64;
                break;
            }
        }
    }
    // println!("visited={:?}", visited);
    // println!("happiness_sorted={:?}", happiness_sorted);
    sum as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        let output = 4;
        let result = maximum_happiness_sum(happiness, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let happiness = vec![2, 3, 4, 5];
        let k = 1;
        let output = 5;
        let result = maximum_happiness_sum(happiness, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let happiness = vec![1, 1, 1, 1];
        let k = 2;
        let output = 1;
        let result = maximum_happiness_sum(happiness, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let happiness = vec![
            2135218, 73431904, 92495076, 77528042, 82824634, 3036629, 28375907, 65220365, 40948869,
            58914871, 57169530, 89783499, 19582915, 19676695, 11932465, 21770144, 49740276,
            22303751, 80746555, 97391584, 95775653, 43396943, 47271136, 43935930, 59643137,
            64183008, 8892641, 39587569, 85086654, 5663585, 82925096, 24868817, 95900395, 48155864,
            74447380, 7618448, 63299623, 91141186, 33347112, 81951555, 52867615, 92184410, 7024265,
            85525916, 29846922, 59532692, 47267934, 6514603, 1137830, 97807470,
        ];

        let k = 41;
        let output = 2517853814;
        let result = maximum_happiness_sum(happiness, k);
        assert_eq!(result, output);
    }
}
