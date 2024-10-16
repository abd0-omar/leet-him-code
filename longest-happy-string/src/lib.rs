pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut heap = std::collections::BinaryHeap::new();
    if a > 0 {
        heap.push((a, 'a'));
    }
    if b > 0 {
        heap.push((b, 'b'));
    }
    if c > 0 {
        heap.push((c, 'c'));
    }
    dbg!(&heap);
    let mut s: Vec<char> = Vec::new();
    while let Some((count_letter, cur_letter)) = heap.pop() {
        if s.len() >= 2 && s[s.len() - 1] == cur_letter && s[s.len() - 2] == cur_letter {
            if let Some(second_big) = heap.pop() {
                s.push(second_big.1);
                if second_big.0 - 1 > 0 {
                    heap.push((second_big.0 - 1, second_big.1));
                }
            } else {
                break;
            }
        }

        s.push(cur_letter);
        if count_letter - 1 > 0 {
            heap.push((count_letter - 1, cur_letter));
        }
    }
    s.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = 1;
        let b = 1;
        let c = 7;
        let output = "ccaccbcc".to_string();
        let result = longest_diverse_string(a, b, c);
        assert_eq!(result, output);
    }
}
