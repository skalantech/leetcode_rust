#![allow(unused)]

#[derive(Clone, Copy, PartialEq)]
enum Item {
    Char(char),
    Empty,
}

struct Stack {
    items: [Item; MAX],
    top: usize,
}

const MAX: usize = 10000;

impl Stack {
    fn new() -> Stack {
        Stack {
            items: [Item::Empty; MAX],
            top: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn is_full(&self) -> bool {
        self.top == MAX
    }

    fn push(&mut self, item: Item) -> bool {
        if self.top < MAX {
            self.items[self.top] = item;
            self.top += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Item {
        if self.top > 0 {
            self.items[self.top - 1]
        } else {
            Item::Char('0') // Possible bug here!
        }
    }

    fn pop(&mut self) -> bool {
        if self.top > 0 {
            self.top -= 1;
            true
        } else {
            false
        }
    }
}

struct Solution {}

impl Solution {
    fn is_valid(&self, s: String) -> bool {
        let mut stack = Stack::new();
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            match chars[i] {
                '(' | '[' | '{' => {
                    stack.push(Item::Char(chars[i]));
                }
                ')' | ']' | '}' => {
                    match chars[i] {
                        ')' => {
                            if stack.peek() == Item::Char('(') {
                                stack.pop();
                            } else {
                                stack.push(Item::Char(chars[i]));
                            }
                        }
                        ']' => {
                            if stack.peek() == Item::Char('[') {
                                stack.pop();
                            } else {
                                stack.push(Item::Char(chars[i]));
                            }
                        }
                        '}' => {
                            if stack.peek() == Item::Char('{') {
                                stack.pop();
                            } else {
                                stack.push(Item::Char(chars[i]));
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            print!("{}", chars[i]);
        }
        stack.is_empty()
    }
}

pub fn main() {
    let s = String::from("{{{[[]]}}}");
    let sol = Solution {};
    let ans = if sol.is_valid(s.clone()) { "true" } else { "false" };
    println!("{}", ans);
}
