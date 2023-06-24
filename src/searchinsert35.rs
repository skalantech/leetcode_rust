pub struct Solution;

impl Solution {
    pub fn search_insert(nums: &Vec<i32>, target: i32) -> i32 {
        let nums_size = nums.len();

        if target == nums[nums_size - 1] {
            return (nums_size - 1) as i32;
        } else if target > nums[nums_size - 1] {
            return nums_size as i32;
        }

        for i in 0..nums_size {
            if nums[i] == target {
                return i as i32;
            } else if target > nums[i] && target < nums[i + 1] {
                return (i + 1) as i32;
            }
        }

        0
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let res = Solution::search_insert(&nums, 4);
    println!("{}", res);
}
