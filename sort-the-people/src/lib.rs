pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    /*
    there are two ways that I know to do this
    to make a tuple and sort
    or
    sort by index
    have a vec with just 0 to n numbers
    then sort them relative to the heights for example
    */
    // let n = heights.len();
    // let mut heights_indeces: Vec<usize> = (0..n).collect();
    // heights_indeces.sort_by(|&a, &b| heights[b].cmp(&heights[a]));
    // let mut result = Vec::with_capacity(n);
    // for i in 0..n {
    //     let sorted_name = &names[heights_indeces[i]];
    //     result.push(sorted_name);
    // }
    // result.into_iter().cloned().collect()

    // sort as a tuple
    let n = heights.len();
    let mut height_name = Vec::with_capacity(n);
    for i in 0..n {
        let cur_height_name = (heights[i], &names[i]);
        height_name.push(cur_height_name);
    }
    height_name.sort_unstable_by(|a, b| b.cmp(a));
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        result.push(height_name[i].1);
    }
    result.into_iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
        let heights = vec![180, 165, 170];
        let output = vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()];
        let result = sort_people(names, heights);
        assert_eq!(result, output);
    }
}
