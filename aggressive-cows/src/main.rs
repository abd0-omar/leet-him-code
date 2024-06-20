// https://www.spoj.com/problems/AGGRCOW/
use std::io::stdin;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    // 1
    // 5 3
    // 1
    // 2
    // 8
    // 4
    // 9
    //
    // t
    // n c
    // ni
    // ni
    // ni
    // ni
    // ni
    // c is the stalls
    // 1, 2, 8, 4, 9 // c -> 3
    // 1, 2, 4, 8, 9 // c -> 3
    // 1, 2, 3, 4, 5, 6, 7, 8, 9
    // x        x           x
    // the min dist is 3 // 4 - 1

    // take input
    let mut t = String::new();
    stdin().read_line(&mut t)?;
    let t: usize = t.trim().parse()?;
    for _ in 0..t {
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        let mut line = line.split_whitespace();
        let n = line.next().unwrap().trim().parse::<usize>()?;
        let c = line.next().unwrap().trim().parse::<i32>()?;
        let mut cows = Vec::with_capacity(n);
        for _ in 0..n {
            let mut ni = String::new();
            stdin().read_line(&mut ni)?;
            let ni = ni.trim().parse::<i32>()?;
            cows.push(ni)
        }

        cows.sort_unstable();
        let mut l = 1;
        let mut r = *cows.last().unwrap();
        let mut ans = -1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if possible(&cows, c, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        println!("{}", ans);
    }

    Ok(())
}

fn possible(cows: &[i32], c: i32, mid: i32) -> bool {
    // put first cow first
    let mut count = 1;
    let mut last_pos = cows[0];

    // we have to place c (assigned stalls) // or m magnetic balls/boys and n is the bins/bathroom stalls
    for &cow in cows.iter().skip(1) {
        if cow - last_pos >= mid {
            count += 1;
            last_pos = cow;
        }
        if count == c {
            return true;
        }
    }
    false
}
