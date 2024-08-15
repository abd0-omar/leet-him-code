#[derive(Debug)]
struct Change {
    five: Option<i32>,
    ten: Option<i32>,
}

impl Change {
    fn new() -> Self {
        Self {
            five: None,
            ten: None,
        }
    }
}

pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut change = Change::new();

    for bill in bills {
        match bill {
            5 => {
                if let Some(five) = change.five {
                    change.five = Some(five + 1);
                } else {
                    change.five = Some(1);
                }
            }
            10 => {
                if let Some(five) = change.five {
                    change.five = Some(five - 1);
                    if let Some(ten) = change.ten {
                        change.ten = Some(ten + 1);
                    } else {
                        change.ten = Some(1);
                    }
                } else {
                    return false;
                }
            }
            20 => {
                if let Some(ten) = change.ten {
                    if let Some(five) = change.five {
                        if ten >= 1 && five >= 1 {
                            change.ten = Some(ten - 1);
                            change.five = Some(five - 1);
                            continue;
                        }
                    }
                }
                if let Some(five) = change.five {
                    if five >= 3 {
                        change.five = Some(five - 3);
                    } else {
                        return false;
                    }
                }
            }
            _ => unreachable!(),
        }
        dbg!(&bill);
        dbg!(&change);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bills = vec![5, 5, 5, 10, 20];
        let output = true;
        let result = lemonade_change(bills);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works1() {
        let bills = vec![5, 5, 10, 10, 20];
        let output = false;
        let result = lemonade_change(bills);
        assert_eq!(result, output);
    }
}
