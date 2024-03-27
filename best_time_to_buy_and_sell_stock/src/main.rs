fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let mut curr_max = 0;
    let mut max = 0;
    let mut st = 0;
    for end in 1..prices.len() {
        curr_max = prices[end] - prices[st];
        println!("curr_max{}", curr_max);
        max = max.max(curr_max);
        println!("max{}", max);

        if prices[end] < prices[st] {
            st = end;
        }
    }
}
