pub struct Solution1;

impl Solution1 {
    fn swap(a: &mut i32, b: &mut i32) {
        let tmp = *a;
        *a = *b;
        *b = tmp;
    }

    fn sort(nums: &mut Vec<i32>, size: usize) {
        for i in 0..size {
            for j in 0..size - i {
                if nums[j] > nums[j + 1] {
                    Self::swap(&mut nums[j], &mut nums[j + 1]);
                }
            }
        }
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = nums.len();

        for i in 0..n {
            for j in 0..n - 1 {
                if i != j + 1 && nums[i] == nums[j + 1] {
                    nums[j + 1] = -1;
                    Self::swap(&mut nums[j + 1], &mut nums[n - 1]);
                    n -= 1;
                }
            }
        }

        Self::sort(nums, n - 1);

        n as i32
    }
}

pub struct Solution2;

impl Solution2 {
    fn swap(a: &mut i32, b: &mut i32) {
        let tmp = *a;
        *a = *b;
        *b = tmp;
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut i = 0;

        while i < n - 1 {
            if nums[i] == nums[i + 1] {
                for j in i..n - 1 {
                    Self::swap(&mut nums[j], &mut nums[j + 1]);
                }
                n -= 1;
            } else {
                i += 1;
            }
        }

        n as i32
    }
}
