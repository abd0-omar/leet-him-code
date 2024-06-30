fn main() {
    use std::io::stdin;
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let m: usize = line.next().unwrap().parse().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let mut a: Vec<char> = line.trim().to_owned().chars().collect();
        // dbg!(&a);
        // array of indices of len n
        let mut indices = Vec::with_capacity(n);
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        for _ in 0..m {
            indices.push(line.next().unwrap().trim().parse::<i32>().unwrap());
        }
        // dbg!(&indices);
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let mut b: Vec<char> = line.trim().to_owned().chars().collect();
        // dbg!(&b);
        // edge cases
        // handle duplicates indices
        indices.sort_unstable();
        b.sort_unstable();
        // put duplicates at the start
        // duplicates of indices
        let mut j = 0;
        for i in 0..m {
            if i != 0 && indices[i] == indices[i - 1] {
                continue;
            }
            // dbg!(&a);
            // dbg!(j);
            a[indices[i] as usize - 1] = b[j as usize];
            j += 1;
        }
        // dbg!(&b);
        // dbg!(&indices);
        // for i in 0..m {
        //     let x = b[i];
        //     a[indices[i] as usize - 1] = x;
        // }
        // dbg!(&a);
        for x in a {
            print!("{}", x);
        }
        println!();
    }
}
