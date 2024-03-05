fn main() {
    println!("Hello, world!");
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 4;
    // Output: "PAHNAPLSIIGYIR"
    println!("{}", convert(s, num_rows));
    // Input: s = "PAYPALISHIRING", numRows = 4
    // Output: "PINALSIGYAHRPI"
    // Explanation:
    // P     I    N
    // A   L S  I G
    // Y A   H R
    // P     I
    // Example 3:
    //
    // Input: s = "A", numRows = 1
    // Output: "A"
}

pub fn convert(s: String, num_rows: i32) -> String {
    let n = s.len();

    let mut i = 0;
    let mut increment = num_rows as usize + 2;

    let mut j_increment = num_rows as usize + 2;
    let mut var = 2;
    while i < num_rows as usize {
        let mut j = i;
        let mut j_increment = i + num_rows as usize + 2;
        while j < n {
            print!("{}, ", &s[j..=j]);
            j += increment;

            if i > 0 {
                // println!("DEBUGPRINT[2]: main.rs:34: j_increment={:#?}", j_increment);
                print!("{}, ", &s[j_increment..=j_increment]);
            }
        }
        i += 1;
        println!();
    }
    println!();
    unimplemented!()
}
