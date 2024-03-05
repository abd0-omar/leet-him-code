fn main() {
    println!("Hello, world!");
}

pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort();
    s.sort();
    let mut j = 0;
    let mut sum = 0;
    for i in 0..g.len() {
        while j < s.len() {
            if s[j] >= g[i] {
                sum += 1;
                j += 1;
                break;
            }
            j += 1;
        }
    }
    sum
}
