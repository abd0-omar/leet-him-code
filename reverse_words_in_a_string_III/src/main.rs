fn reverse_words(s: String) -> String {
    let mut st: Vec<char> = vec![];
    let mut res = String::new();

    for c in s.chars() {
        if c == ' ' {
            while let Some(x) = st.pop() {
                res.push(x);
            }
            res.push(' ');
            continue;
        }
        st.push(c);
    }
    while let Some(x) = st.pop() {
        res.push(x);
    }
    res
}

fn main() {
    println!("Hello, world!");
    let str = String::from("hello world");
    println!("{}", reverse_words(str));
}
