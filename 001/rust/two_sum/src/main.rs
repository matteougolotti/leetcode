/*

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

*/

use std::collections::HashMap;


fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut numbers = HashMap::new();

    for (i, number) in nums.iter().enumerate() {
        if !numbers.contains_key(number) {
            numbers.insert(number, i);
        }
    }

    for (i, number) in nums.iter().enumerate() {
        let first = number;
        let second = target - first;
        if numbers.contains_key(&second) {
            let first_index = i as i32;
            let second_index = *numbers.get(&second).unwrap() as i32;
            if first_index != second_index {
                return vec![first_index, second_index];
            }
        }
    }

    return vec![];
}

fn main() {
    let nums_1 = vec![2,7,11,15];
    let res_1 = two_sum(nums_1, 9);
    println!("Result 1 -> {:?}", res_1);
}
