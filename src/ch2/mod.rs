// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn add(&mut self, val: i32) -> &mut Self {
        self.next = Some(Box::new(ListNode { val, next: None }));
        self
    }

    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut current_node = &mut head;

        let mut carry = 0;
        let mut l1_ptr = &l1;
        let mut l2_ptr = &l2;

        loop {
            let num1 = l1_ptr.as_ref().and_then(|node| Some(node.val));
            let num2 = l2_ptr.as_ref().and_then(|node| Some(node.val));
            if num1.is_none() && num2.is_none() {
              // We can guarantee that if carry is non zero, current_node is not empty
              if carry != 0 {
                current_node.as_mut().unwrap().add(carry);
              }

              break;
            }

            let sum = num1.unwrap_or(0) + num2.unwrap_or(0) + carry;
            carry = sum / 10;
            let digit = sum % 10;

            match current_node {
                Some(node) => {
                    node.next = Some(Box::new(ListNode::new(digit)));
                    current_node = &mut node.next;
                }
                None => {
                    *current_node = Some(Box::new(ListNode::new(digit)));
                }
            }

            l1_ptr = match l1_ptr {
                Some(l1_box) => &l1_box.next,
                None => &None,
            };
            l2_ptr = match l2_ptr {
                Some(l2_box) => &l2_box.next,
                None => &None,
            };
        }
        head
    }
}
