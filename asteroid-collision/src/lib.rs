pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    // parse them with l and r
    let n = asteroids.len();
    let mut asteroids_dir = Vec::with_capacity(n);
    for &asteroid in asteroids.iter() {
        if asteroid > 0 {
            asteroids_dir.push((asteroid, 'R'));
        } else if asteroid < 0 {
            asteroids_dir.push((-asteroid, 'L'));
        } else {
            panic!("asteroid equals zero");
        }
    }
    let mut stack: Vec<(i32, char)> = vec![];
    'outer: for (asteroid, dir) in asteroids_dir {
        while !stack.is_empty() {
            if stack.last().unwrap().1 == 'R' && dir == 'L' {
                if asteroid == stack.last().unwrap().0 {
                    stack.pop();
                    continue 'outer;
                } /* else */
                if stack.last().unwrap().0 > asteroid {
                    continue 'outer;
                } /* else */
                {
                    stack.pop();
                }
            } else {
                stack.push((asteroid, dir));
                continue 'outer;
            }
        }
        stack.push((asteroid, dir));
    }
    //println!("{:?}", stack);
    // return back the '-' negative signs instead of 'L'
    for (asteroid, dir) in stack.iter_mut() {
        if *dir == 'L' {
            *asteroid *= -1;
        }
    }
    stack.into_iter().map(|(x, _)| x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let asteroids = vec![5, 10, -5];
        let output = vec![5, 10];
        let result = asteroid_collision(asteroids);
        assert_eq!(result, output);
    }
}
