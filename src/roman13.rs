#![allow(unused)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    fn roman_to_int(s: String) -> i32 {
        let mut roman: HashMap<char, i32> = HashMap::new();
        roman.insert('I', 1);
        roman.insert('V', 5);
        roman.insert('X', 10);
        roman.insert('L', 50);
        roman.insert('C', 100);
        roman.insert('D', 500);
        roman.insert('M', 1000);

        let mut integer = 0;
        let n = s.len();
        let mut i = 0;
        while i < n {
            if i < n - 1 {
                if s.chars().nth(i).unwrap() == 'I' && (s.chars().nth(i + 1).unwrap() == 'V' || s.chars().nth(i + 1).unwrap() == 'X') {
                    roman.insert('I', -1);
                }
                if s.chars().nth(i).unwrap() == 'X' && (s.chars().nth(i + 1).unwrap() == 'L' || s.chars().nth(i + 1).unwrap() == 'C') {
                    roman.insert('X', -10);
                }
                if s.chars().nth(i).unwrap() == 'C' && (s.chars().nth(i + 1).unwrap() == 'D' || s.chars().nth(i + 1).unwrap() == 'M') {
                    roman.insert('C', -100);
                }
            }
            integer += roman.get(&s.chars().nth(i).unwrap()).unwrap();
            roman.insert('I', 1);
            roman.insert('X', 10);
            roman.insert('C', 100);
            i += 1;
        }
        integer
    }
}

pub fn main() {
    let str_value = String::from("MCMXCIV");
    let res = Solution::roman_to_int(str_value);
    println!("{}", res);
}
