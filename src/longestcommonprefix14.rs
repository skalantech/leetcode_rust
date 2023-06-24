#![allow(unused)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let n_words = strs.len();
        let mut pattern = String::new();

        if n_words == 0 {
            return pattern;
        } else if n_words == 1 {
            return strs[0].clone();
        } else {
            let mut word = strs[0].clone();
            let mut shortest = word.len();

            for g in 1..n_words {
                if strs[g].len() < shortest {
                    shortest = strs[g].len();
                }
            }
            //println!("{}", shortest);
            let mut y = shortest;
            for i in 1..n_words {
                let other = &strs[i];
                let mut k = 0;

                for j in 0..shortest {
                    if word.chars().nth(j).unwrap() == other.chars().nth(j).unwrap() {
                        k += 1;
                    } else {
                        break;
                    }
                }
                
                if k < y {
                    y = k;
                }
                //println!("{}", y);
            }
            
            for h in 0..y {
                    pattern.push(word.chars().nth(h).unwrap());
            }

            pattern
        }
    }
}

pub fn main() {
    let mut strs = vec!("dog","racecar","car")
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();
    let res = Solution::longest_common_prefix(strs);
    println!("{}", res);
}
