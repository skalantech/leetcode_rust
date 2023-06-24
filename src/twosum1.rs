struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        let mut res = Vec::new();

        for i in 0..n {
            for j in (i + 1)..n {
                if target == nums[i] + nums[j] {
                    res.push(i as i32);
                    res.push(j as i32);
                }
            }
        }
        res
    }
}

pub fn main() {
    let nums = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    let target = 20;
    let res = Solution::two_sum(nums, target);
    println!("{:?}", res);
}