fn main() {
    println!("Hello, world!");
    let s = "tree".to_string();
    // Output: "eert"
    println!("{}", frequency_sort(s));
    let s = "Aabb".to_string();
    // Output: "aaaccc"
    println!("{}", frequency_sort(s));
}

pub fn frequency_sort(s: String) -> String {
    let mut hm = std::collections::HashMap::new();
    for c in s.chars() {
        *hm.entry(c).or_insert(0) += 1;
    }

    let mut sorted_vec = hm.iter().collect::<Vec<_>>();
    sorted_vec.sort_unstable_by(|(_, x_val), (_, y_val)| y_val.cmp(x_val));
    let mut rez = String::with_capacity(s.len());
    for (&c, &freq) in sorted_vec {
        for _ in 0..freq {
            rez.push(c);
        }
    }
    rez
}
