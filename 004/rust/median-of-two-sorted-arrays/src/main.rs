
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums3 = Vec::new();

    let mut index_1 = 0;
    let mut index_2 = 0;
    while index_1 + index_2 < nums1.len() + nums2.len() {
        if index_1 >= nums1.len() {
            nums3.push(nums2[index_2]);
            index_2 = index_2 + 1;
        } else if index_2 >= nums2.len() {
            nums3.push(nums1[index_1]);
            index_1 = index_1 + 1;
        } else if nums1[index_1] <= nums2[index_2] {
            nums3.push(nums1[index_1]);
            index_1 = index_1 + 1;
        } else {
            nums3.push(nums2[index_2]);
            index_2 = index_2 + 1;
        }
    }

    return match nums3.len() % 2 {
        0 => (nums3[nums3.len() / 2 - 1] as f64 + nums3[nums3.len() / 2] as f64) / 2.0,
        _ => nums3[nums3.len() / 2] as f64,
    };
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];

    let result = find_median_sorted_arrays(nums1, nums2);

    println!("Result {}", result);
}
