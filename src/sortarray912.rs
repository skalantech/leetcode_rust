#![allow(unused)]
pub struct Solution;

impl Solution {
    pub fn sort_array(nums: &mut Vec<i32>) -> &mut Vec<i32> {
        let size = nums.len();

        if size > 1 {
            // let l = 0;
            let m = size / 2;
            let r = size;

            let mut ll = Vec::new();
            let mut mm = Vec::new();

            for i in 0..m {
                ll.push(nums[i]);
            }
            for i in m..r {
                mm.push(nums[i]);
            }

            Solution::sort_array(&mut ll);
            Solution::sort_array(&mut mm);

            let mut i = 0;
            let mut j = 0;
            let mut k = 0;

            while i < ll.len() && j < mm.len() {
                if ll[i] < mm[j] {
                    nums[k] = ll[i];
                    i += 1;
                } else {
                    nums[k] = mm[j];
                    j += 1;
                }
                k += 1;
            }

            while i < ll.len() {
                nums[k] = ll[i];
                i += 1;
                k += 1;
            }

            while j < mm.len() {
                nums[k] = mm[j];
                j += 1;
                k += 1;
            }
        }

        nums
    }
}

pub fn main() {
    let mut data = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    Solution::sort_array(&mut data);
    println!("{:?}", &data);
}
