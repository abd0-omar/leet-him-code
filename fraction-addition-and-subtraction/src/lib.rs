pub fn fraction_addition(expression: String) -> String {
    let expression = if expression.as_bytes()[0] != b'-' {
        format!("+{}", expression)
    } else {
        expression
    };

    let mut total = (0, 1); // (numerator, denominator)
    let mut i = 0;
    let n = expression.len();

    while i < n {
        let mut j = i + 1;
        while j < n && expression.as_bytes()[j] != b'+' && expression.as_bytes()[j] != b'-' {
            j += 1;
        }

        let fraction_str = &expression[i..j];
        let parts: Vec<&str> = fraction_str[1..].split('/').collect();
        let numerator: i32 = parts[0].parse().unwrap();
        let denominator: i32 = parts[1].parse().unwrap();

        let sign = if fraction_str.starts_with('-') { -1 } else { 1 };
        let fraction = (numerator * sign, denominator);

        total = add_fractions(total, fraction);

        i = j;
    }

    format!("{}/{}", total.0, total.1)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn add_fractions((n1, d1): (i32, i32), (n2, d2): (i32, i32)) -> (i32, i32) {
    let numerator = n1 * d2 + n2 * d1;
    let denominator = d1 * d2;
    let gcd_value = gcd(numerator, denominator);
    (numerator / gcd_value, denominator / gcd_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let expression = "-1/2+1/2".to_string();
        let output = "0/1".to_string();
        let result = fraction_addition(expression);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let expression = "1/3-1/2".to_string();
        let output = "-1/6".to_string();
        let result = fraction_addition(expression);
        assert_eq!(result, output);
    }
}
