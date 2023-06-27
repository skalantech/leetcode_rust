#![allow(unused)]
struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut n = nums.len();
        let mut i = 0;
        
        while i < n {
            if nums[i] == val {
                for j in i..n-1 {
                    nums[j] = nums[j + 1];
                }
                n -= 1;
                i -= 1;
            }
            i += 1;
        }
        n as i32
    }
}

pub fn main() {
    let mut data = vec![0,1,2,2,3,0,4,2];
    let val = 2;
    let ans = Solution::remove_element(&mut data, val);
    println!("new vector: {:?} and len: {}", &data, ans);
    for _ in 0..data.len()-ans as usize {
        data.remove(ans as usize);
    }
    println!("new vector: {:?} and len: {}", &data, ans);
}