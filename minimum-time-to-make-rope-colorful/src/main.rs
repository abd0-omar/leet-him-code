fn main() {
    println!("Hello, world!");
}

pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    _min_cost(colors.as_bytes(), &needed_time, 0, None)
}

pub fn _min_cost(colors: &[u8], needed_time: &Vec<i32>, idx: usize, prv: Option<usize>) -> i32 {
    if idx >= colors.len() {
        return 0;
    }

    // pick
    let choice2;
    if let Some(p) = prv {
        if colors[p] == colors[idx] {
            let min = needed_time[p].min(needed_time[idx]);
            if needed_time[p] < needed_time[idx] {
                choice2 = needed_time[p] + _min_cost(colors, needed_time, idx + 1, Some(idx));
            } else {
                choice2 = needed_time[idx] + _min_cost(colors, needed_time, idx + 2, Some(idx + 1));
            }
        } else {
            choice2 = _min_cost(colors, needed_time, idx + 1, Some(idx));
        }
    } else {
        choice2 = _min_cost(colors, needed_time, idx + 1, Some(idx));
    }

    return choice2;
}
