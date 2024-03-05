use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut st: Vec<char> = vec![];
    let mut hm: HashMap<char, char> = HashMap::new();
    hm.insert(')', '(');
    hm.insert(']', '[');
    hm.insert('}', '{');
    for i in s.chars() {
        if hm.contains_key(&i) {
            if st.last() == Some(&hm[&i]) {
                st.pop();
                continue;
            } else {
                return false;
            }
        }
        st.push(i);
    }
    if st.is_empty() {
        return true;
    }
    false
}
fn main() {
    let s: String = String::from("]");
    print!("{}", is_valid(s));
}
