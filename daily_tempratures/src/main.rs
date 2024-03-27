fn main() {
    let temperatures: Vec<i32> = vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70];
    let mut st: Vec<(i32, usize)> = vec![];

    let mut res: Vec<i32> = vec![0; temperatures.len()];

    for i in 0..temperatures.len() {
        while st.last().is_some() && temperatures[i] > st.last().unwrap().0 {
            let rez = i - st.last().unwrap().1;
            res[st.last().unwrap().1] = rez as i32;

            st.pop();
        }

        st.push((temperatures[i], i));
    }

    println!("{:?}", res);
}
