fn main() {
    println!("Hello, world!");
}

pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by(|a, b| a[0].cmp(&b[0]));
    println!("points={:?}", points);
    let mut max = 0;
    for i in 1..points.len() {
        let diff = points[i][0] - points[i - 1][0];
        if diff > max {
            max = diff;
        }
    }
    max
}
