fn main() {
    println!("Hello, world!");
    use std::io::stdin;
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let a: i32 = line.next().unwrap().parse().unwrap();
        let b: i32 = line.next().unwrap().parse().unwrap();
        let c: i32 = line.next().unwrap().parse().unwrap();
        let vec = vec![a, b, c];
        // try all
        let mut min = i32::MAX;
        for i in 0..3 {
            let mut cur_min = 0;
            for &x in vec.iter() {
                cur_min += (vec[i] - x).abs();
            }
            min = min.min(cur_min);
        }
        println!("{}", min);
    }
}
