fn main() {
    println!("Hello, world!");
    let nums1 = vec![34, 225, 328, 530, 823, 834, 902, 989];
    let nums2 = vec![
        24, 30, 115, 121, 160, 173, 239, 265, 335, 362, 449, 557, 597, 624, 697, 766, 775, 881,
        898, 919,
    ];
    println!("{}", get_common(nums1, nums2));
}

pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    if nums1[nums1.len() - 1] < nums2[0] || nums1[0] > nums2[nums2.len() - 1] {
        return -1;
    }
    while x < nums1.len() && y < nums2.len() {
        println!("y={:?}", y);
        if nums2[y] > nums1[x] {
            x += 1;
        } else if nums2[y] < nums1[x] {
            y += 1;
        } else {
            return nums1[x];
        }
    }
    -1
}
