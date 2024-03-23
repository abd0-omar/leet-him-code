fn main() {
    println!("Hello, world!");
    let n = 8;
    println!("{}", pivot_integer(n));
}

pub fn pivot_integer(n: i32) -> i32 {
    let n = n as usize;
    let mut sum_vec = vec![0; n + 1];
    for i in 1..=n {
        sum_vec[i] = sum_vec[i - 1] + i;
    }
    // println!("sum_vec={:?}", sum_vec);
    for i in 0..n {
        let sum1 = sum_vec[i + 1];
        // println!("sum1={:?}", sum1);
        let sum2 = sum_vec[n] - sum_vec[i];
        // println!("sum2={:?}", sum2);
        if sum1 == sum2 {
            return i as i32 + 1;
        }
    }
    -1
}
