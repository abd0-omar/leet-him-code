use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut hs: HashSet<i32> = HashSet::new();
    let mut b: BTreeSet<i32> = BTreeSet::new();
    let mut bm = BTreeMap::new();

    for e in nums.iter() {
        let count = hm.entry(*e).or_insert(1);
        *count += 1;
        hs.insert(*e);
    }

    let mut stack: Vec<i32> = Vec::new();
    let mut hm2: HashMap<i32, Vec<i32>> = HashMap::new();

    for e in hm.iter() {
        b.insert(*e.1);
        // hm2.insert(*e.1, *e.0);
        let v = hm2.entry(*e.1).or_insert(Vec::new()).push(*e.0);

        bm.entry(-*e.1).or_insert(Vec::new()).push(*e.0);

        // if stack.len() < k as usize {
        //     stack.push(*e);
        //     continue;
        // }
        //
        // if hm.get(stack.last().unwrap()) > hm.get(e) {
        //     continue;
        // } else {
        //     unimplemented!()
        // }
    }

    let mut rez = vec![];
    let mut i = 0;
    for e in bm.iter() {
        if i == k {
            break;
        }

        rez.extend_from_slice(e.1);
        i += e.1.len() as i32;
    }
    rez

    // println!("{:?}", hm);
    // println!("{:?}", hm2);
    //
    // let mut rez: Vec<i32> = vec![];
    // let mut i = 0;
    // //very cool trick to iterate over a vec inside a hashmap
    // for &count in b.iter().rev() {
    //     if i == k {
    //         break;
    //     }
    //
    //     if let Some(elements) = hm2.get(&count) {
    //         rez.extend_from_slice(elements);
    //         i += elements.len() as i32;
    //     }
    // }
    //
    // // for key in hm.keys() {
    // //     v.push(key);
    // // }
    //
    // // rez
    // rez
}
fn main() {
    let ex1 = vec![1, 2];
    let ex2 = vec![1, 1, 1, 2, 2, 3, 3, 4, 4, 3, 3, 2, 2, 2, 2];

    println!("{:?}", top_k_frequent(ex2, 4));
}
