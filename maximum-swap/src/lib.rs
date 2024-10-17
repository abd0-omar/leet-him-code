pub fn maximum_swap(num: i32) -> i32 {
    // a7eh kont hansa el daily leetcode
    // y5rbet om el kolya
    // ana kont 7aletha fe dma8y w ana ray7 el kolya
    // w kanet sahla awe k implementation w el fkra
    // 3ks emabare7 elimplementation bta3ha m4 altf 7aga
    // a7a ana b2et bktb franko
    // el mohm hya htb2a prefix max bel3ks
    // w fe el pass el tanya, hbdel b 2wl rkm akbr mny
    // w ana lsa btklem franko
    let mut num = num.to_string().into_bytes();
    // dbg!(&num);
    let n = num.len();
    let mut prefix_max = vec![(i32::MIN, 0); n + 1];
    for i in (0..n).rev() {
        // dbg!(i);
        if prefix_max[i + 1].0 < (num[i] - b'0') as i32 {
            prefix_max[i] = ((num[i] - b'0') as i32, i);
        } else {
            prefix_max[i] = prefix_max[i + 1];
        }
    }
    // dbg!(&prefix_max);
    // second pass
    for i in 0..n {
        if ((num[i] - b'0') as i32) < prefix_max[i].0 {
            let temp = num[i];
            num[i] = prefix_max[i].0 as u8 + b'0';
            num[prefix_max[i].1] = temp;
            break;
        }
    }
    // dbg!(&num);
    // who said rust is verbose
    String::from_utf8(num).unwrap().parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let num = 2736;
        let output = 7236;
        let result = maximum_swap(num);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let num = 98368;
        let output = 98863;
        let result = maximum_swap(num);
        assert_eq!(result, output);
    }
}
