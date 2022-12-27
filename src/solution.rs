#![allow(unused)]
use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    // 构造方法，返回空对象
    pub fn new() -> Solution {
        Solution {}
    }
    // &self 自身引用，不转移所有权
    /**
    求
     */
    pub fn _contains_duplicate(&self, nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        nums.iter().any(|&x| !set.insert(x))
        // let a:std::collections::BTreeSet<&i32> = nums.iter().collect();
        // a.len() < nums.len()
    }
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut res = nums[0];
        for i in nums {
            if sum > 0 {
                sum += i;
            } else {
                sum = i;
            }
            res = std::cmp::max(res, sum);
        }
        res
    }
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if let Some(val) = map.get(&(target - nums[i])) {
                if i != *val {
                    return vec![i as i32, *val as i32];
                }
            }
            map.insert(nums[i], i);
        }
        Vec::new()
    }
}
