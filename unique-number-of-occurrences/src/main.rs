fn main() {
    println!("Hello, world!");
}

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut freq = std::collections::HashMap::with_capacity(arr.len());
    for num in arr {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut freq_sorted = freq.into_values().collect::<Vec<i32>>();
    freq_sorted.sort();

    for i in 1..freq_sorted.len() {
        if freq_sorted[i] == freq_sorted[i - 1] {
            return false;
        }
    }
    true
}
