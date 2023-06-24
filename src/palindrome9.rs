#![allow(unused)]

pub struct Solution1;

impl Solution1 {
    pub fn is_palindrome(x: i32) -> bool {
        let entry = x.to_string();
        let mut rev = String::with_capacity(entry.len());
        let mut res = false;

        for c in entry.chars().rev() {
            rev.push(c);
        }
        //println!("{}, {}", entry, rev);
        if entry == rev {
            res = true;
        }

        res
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn is_palindrome(x: i32) -> bool {
        let entry = x.to_string();
        let rev: String = entry.chars().rev().collect();
        entry == rev
    }
}

pub struct Solution3;

impl Solution3 {
    pub fn is_palindrome(x: i32) -> bool {
        let entry = x.to_string();
        let rev = entry.chars().rev().collect::<String>();
        entry == rev
    }
}

pub fn main() {
    let x = -13251;
    let res1 = Solution1::is_palindrome(x);
    let res2 = Solution2::is_palindrome(x);
    let res3 = Solution3::is_palindrome(x);
    println!("{}", res1);
    println!("{}", res2);
    println!("{}", res3);
}