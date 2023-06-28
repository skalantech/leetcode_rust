#![allow(unused)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn append(&mut self, val: i32) {
        let new_node = Box::new(ListNode::new(val));
        let mut current_node = self;

        while let Some(ref mut next_node) = current_node.next {
            current_node = next_node;
        }

        current_node.next = Some(new_node);
    }

    fn print(&self) {
        let mut current_node = self;

        while let Some(next_node) = &current_node.next {
            println!("{:?}", current_node.val);
            current_node = next_node;
        }

        println!("{:?}", current_node.val);
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut p = l1;
        let mut q = l2;

        let mut carry: i32 = 0;

        while p != None || q != None {
            let sum = match (&p, &q) {
                (Some(l1), Some(l2)) => l1.val + l2.val + carry,
                (Some(l1), None) => l1.val + carry,
                (None, Some(l2)) => l2.val + carry,
                (None, None) => carry,
            };

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            p = if p != None { p.unwrap().next } else { p };
            q = if q != None { q.unwrap().next } else { q };
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy_head.next
    }
}

pub fn main() {
    let mut l1= Box::new(ListNode::new(2));
    l1.append(6);
    l1.append(9);
    println!("l1 list:");
    l1.print();

    let mut l2= Box::new(ListNode::new(2));
    l2.append(6);
    l2.append(9);
    println!("l2 list:");
    l2.print();

    let l3 = Solution::add_two_numbers(Some(l1), Some(l2));
    println!("l3 list:");
    l3.unwrap().print();
}
