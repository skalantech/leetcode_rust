#![allow(unused)]
pub struct Solution1;

impl Solution1 {
    // fn swap(a: &mut i32, b: &mut i32) {
    //     let tmp = *a;
    //     *a = *b;
    //     *b = tmp;
    // }

    // fn sort(nums: &mut Vec<i32>, size: usize) {
    //     for i in 0..size {
    //         for j in 0..size - i {
    //             if nums[j] > nums[j + 1] {
    //                 nums.swap(j, j + 1);
    //             }
    //         }
    //     }
    // }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut i: i32 = 0;
        
        while (i as usize) < n - 1 {
            if nums[i as usize] == nums[i as usize + 1] {
                for j in (i as usize)..n-1 {
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

pub struct Solution2;

impl Solution2 {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut i = 0;

        while i < n - 1 {
            if nums[i] == nums[i + 1] {
                for j in i..n - 1 {
                    nums.swap(j, j + 1);
                }
                n -= 1;
            } else {
                i += 1;
            }
        }
        n as i32
    }
}

pub fn main() {
    let mut data0 = vec!(0,0,1,1,1,2,2,3,3,4);
    let data1 = [0,0,1,1,1,2,2,3,3,4];
    let mut data2: Vec<i32> = Vec::new();
    for i in 0..data1.len() {
        data2.push(data1[i]);
    }
    let sol1 = Solution1::remove_duplicates(&mut data0);
    let sol2 = Solution2::remove_duplicates(&mut data2);
    println!("{:?}, answer = {}", &data0, sol1);
    println!("{:?}, answer = {}", &data2, sol2);
    for _ in 0..data2.len()-sol2 as usize {
        data2.remove(sol2 as usize);
    }
    println!("{:?}, answer = {}", &data2, sol2);
}