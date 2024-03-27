fn main() {
    let piles = vec![3, 6, 7, 11];
    let h = 8;

    println!("{}", min_eating_speed(piles, h));
}

fn possible(mid: i32, piles: &Vec<i32>, h: i32) -> bool {
    if mid == 0 {
        return false;
    }
    // let mut c = 0;
    //
    // for pile in piles {
    //     let mut f = *pile;
    //     c += 1;
    //     while f > mid {
    //         println!("DEBUGPRINT[2]: main.rs:14: f={:#?}", f);
    //         c += 1;
    //         f -= mid;
    //     }
    // }
    // println!("DEBUGPRINT[1]: main.rs:20: mid={:#?}", mid);
    // println!("DEBUGPRINT[1]: main.rs:20: c={:#?}", c);
    //
    // println!("should get the same results");
    let mut c = 0;

    for pile in piles {
        c += (pile + mid - 1) / mid;
    }

    // println!("DEBUGPRINT[1]: main.rs:20: mid={:#?}", mid);
    // println!("DEBUGPRINT[1]: main.rs:20: c={:#?}", c);
    c <= h
}

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut l = 0;
    let mut r = *piles.iter().max().unwrap();
    let mut ans = -1;
    // println!("possible {}", possible(4, &piles, h));

    while l <= r {
        let mid = l + (r - l) / 2;
        if possible(mid, &piles, h) {
            r = mid - 1;
            ans = mid;
        } else {
            l = mid + 1;
        }
    }

    ans
}
