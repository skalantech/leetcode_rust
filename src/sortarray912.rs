pub struct Solution;

impl Solution {
    pub fn sort_array(nums: &mut Vec<i32>) -> &mut Vec<i32> {
        let size = nums.len();
        
        if size > 1 {
            let l = 0;
            let m = size / 2;
            let r = size;

            let mut L = Vec::new();
            let mut M = Vec::new();

            for i in 0..m {
                L.push(nums[i]);
            }
            for i in m..r {
                M.push(nums[i]);
            }

            Solution::sort_array(&mut L);
            Solution::sort_array(&mut M);

            let mut i = 0;
            let mut j = 0;
            let mut k = 0;

            while i < L.len() && j < M.len() {
                if L[i] < M[j] {
                    nums[k] = L[i];
                    i += 1;
                } else {
                    nums[k] = M[j];
                    j += 1;
                }
                k += 1;
            }

            while i < L.len() {
                nums[k] = L[i];
                i += 1;
                k += 1;
            }

            while j < M.len() {
                nums[k] = M[j];
                j += 1;
                k += 1;
            }
        }
        
        nums
    }
}
