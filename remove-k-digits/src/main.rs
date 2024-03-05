fn main() {
    let num = "1432219".to_string();
    let k = 3;

    let num2 = String::from("10200");

    let s = remove_kdigits(num, k);
    println!("{s}");
}

pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut rez: String = String::with_capacity(num.len());
    let mut n = 0;

    for c in num.chars() {
        while !rez.is_empty() && n < k && rez.chars().last().unwrap() > c {
            rez.pop();
            n += 1;
        }

        if rez.is_empty() && c == '0' {
            continue;
        } else {
            rez.push(c);
        }
    }

    while n < k {
        rez.pop();
        n += 1;
    }

    if rez.is_empty() {
        return "0".to_owned();
    }

    rez
}
