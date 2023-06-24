pub struct Solution1;

impl Solution1 {
    pub fn is_palindrome(x: i32) -> bool {
        let entry = x.to_string();
        let mut rev = String::with_capacity(entry.len());
        let mut res = false;

        for c in entry.chars().rev() {
            rev.push(c);
        }

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
        let mut rev = entry.clone();
        let mut res = false;

        rev.chars().rev();

        if entry == rev {
            res = true;
        }

        res
    }
}

pub struct Solution3;

impl Solution3 {
    pub fn is_palindrome(x: i32) -> bool {
        let entry = x.to_string();
        let rev: String = entry.chars().rev().collect();
        entry == rev
    }
}

pub struct Solution4;

impl Solution4 {
    pub fn is_palindrome(x: i32) -> bool {
        let entry = x.to_string();
        let rev = entry.chars().rev().collect::<String>();
        entry == rev
    }
}

