fn main() {
    println!("Hello, world!");
    let paths = vec![
        vec!["London".to_string(), "New York".to_string()],
        vec!["New York".to_string(), "Lima".to_string()],
        vec!["Lima".to_string(), "Sao Paulo".to_string()],
    ];
    dest_city(paths);
}

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut hm = std::collections::HashMap::new();

    for path in paths.iter() {
        let from = &path[0];
        let to = &path[1];
        hm.insert(from, to);
    }
    // println!("hm={:?}", hm);

    let mut result = String::new();

    dfs(&paths, paths[0][0].clone(), &mut result, &mut hm);

    // println!("result={:?}", result);
    // no loops == no need for visited

    result
}

fn dfs(
    paths: &Vec<Vec<String>>,
    curr: String,
    result: &mut String,
    hm: &mut std::collections::HashMap<&String, &String>,
) {
    *result = curr.clone();
    if let Some(neighbor) = hm.get(&curr) {
        // println!("neighbor={:?}", neighbor);
        dfs(paths, neighbor.to_string(), result, hm);
    }
}
