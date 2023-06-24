use std::collections::HashMap;

struct Solution;

impl Solution {
    fn longest_common_prefix(strs: &mut Vec<String>) -> String {
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

                for _ in 0..k {
                    strs[i].remove(0);
                }
            }

            if let Some(min) = strs.iter().map(|s| s.len()).min() {
                for _ in 0..min {
                    pattern.push(word.chars().nth(0).unwrap());
                    word.remove(0);
                }
            }

            pattern
        }
    }
}

fn main() {
    let mut strs = vec!["flower", "flow", "flight"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();
    let res = Solution::longest_common_prefix(&mut strs);
    println!("{}", res);
}
