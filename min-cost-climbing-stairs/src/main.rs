fn main() {
    println!("Hello, world!");
    // let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    // Output: 6

    // let cost = vec![10, 15, 20];
    // // Output: 15
    //
    // let cost = vec![1, 2, 2, 0];
    // // Output: 2
    //
    let cost = vec![0, 0, 1, 1];
    // Output: 0
    println!("{}", min_cost_climbing_stairs(cost));
}
//
// Input: cost = [10,15,20]
// Output: 15

pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
    let mut sum = 0;

    let mut i = 1;

    while i < cost.len() {
        sum += cost[i].min(cost[i - 1]);
        cost[i] = cost[i].min(cost[i - 1]);
        i += 1;
    }

    sum
}
// pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
//     let mut sum = 0;
//     let mut i = 0;
//     while i < cost.len() - 1 {
//         println!("DEBUGPRINT[5]: main.rs:12: i={:#?}", i);
//         // -4
//         //
//         if cost.len() >= 4 && i == cost.len() - 4 {
//             println!("hello");
//             let min1 = cost[i] + cost[i + 2];
//             let min2 = cost[i + 1];
//             let min3 = (cost[i + 1] + cost[i + 3]).min(min2);
//
//             sum += min1.min(min3);
//             break;
//         }
//
//         if cost.len() >= 3 && i == cost.len() - 3 {
//             println!("helloz");
//             let min1 = cost[i] + cost[i + 2];
//             let min2 = cost[i + 1];
//
//             sum += min1.min(min2);
//             break;
//         }
//         let min1 = cost[i];
//         println!("DEBUGPRINT[1]: main.rs:18: min1={:#?}", min1);
//         let min2 = cost[i + 1];
//         println!("DEBUGPRINT[2]: main.rs:20: min2={:#?}", min2);
//
//         if min1 < min2 {
//             sum += cost[i];
//         // } else if min1 > min2 {
//         } else {
//             sum += cost[i + 1];
//             i += 1;
//         }
//         println!("DEBUGPRINT[4]: main.rs:28: i={:#?}", i);
//         println!("DEBUGPRINT[3]: main.rs:27: sum={:#?}", sum);
//         i += 1;
//     }
//     // if i == len - 3
//     // choose: cost[i] + cost[i + 2] or cost[i + 1]
//     sum
// }
