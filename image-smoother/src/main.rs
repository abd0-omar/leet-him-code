fn main() {
    println!("Hello, world!");
}

pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (img.len(), img[0].len());
    let mut res = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            //get avg of surroundings
            let mut sum = img[i][j];
            let mut count = 1;
            for (di, dj) in [
                (0, 1),
                (1, 0),
                (-1, 0),
                (0, -1),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ] {
                let di = (i as i32 + di) as usize;
                let dj = (j as i32 + dj) as usize;
                if (0..m).contains(&di) && (0..n).contains(&dj) {
                    sum += img[di][dj];
                    count += 1;
                }
            }
            res[i][j] = sum / count;
        }
    }
    res
}
