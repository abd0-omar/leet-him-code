fn main() {
    let s = "deeedbbcccbdaa".to_string();
    let k = 3;

    let rez = remove_duplicates(s, k);
    println!("{rez}");
}

pub fn remove_duplicates(s: String, k: i32) -> String {
    let mut st: Vec<(char, i32)> = Vec::new();

    for c in s.chars() {
        if let Some((top_char, top_count)) = st.last_mut() {
            if *top_char == c {
                *top_count += 1;
                if *top_count == k {
                    st.pop();
                }
            } else {
                st.push((c, 1));
            }
        } else {
            st.push((c, 1));
        }
    }

    println!("{:?}", st);

    let mut rez: String = String::new();
    while !st.is_empty() {
        if let Some((char, count)) = st.last_mut() {
            while *count > 0 {
                println!("{}", char);
                rez.push(*char);
                *count -= 1;
            }
            st.pop();
        }
    }
    println!("finish");
    rez.chars().rev().collect()
}
