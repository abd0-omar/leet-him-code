fn main() {
    let nums1: Vec<i32> = vec![4, 9, 5];
    let nums2: Vec<i32> = vec![9, 4, 9, 8];

    let mut freq1: Vec<Option<i32>> = vec![None; 1000];
    let mut freq2: Vec<Option<i32>> = vec![None; 1000];

    let mut res: Vec<i32> = vec![];

    for &c in &nums1 {
        freq1[c as usize] = Some(match freq1[c as usize] {
            Some(val) => val + 1,
            None => 1,
        });
    }

    for c in nums1 {
        // freq1[c as usize] = Some(match freq1[c as usize] {
        //     Some(val) => val + 1,
        //     None => 1,
        // });
        println!("{}", c);
    }

    for &c in &nums2 {
        freq2[c as usize] = Some(match freq2[c as usize] {
            Some(num) => num + 1,
            None => 1,
        });
    }

    for i in 0..1000 {
        if freq1[i].is_some() && freq2[i].is_some() {
            if let Some(_) = freq1[i] {
                res.push(i as i32);
            }
        }
    }

    println!("{:?}", res);
}
