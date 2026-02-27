struct Draft;
struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Draft {
    /// draft version with corrected parity.  the original counted from 0 and
    /// therefore produced even-first order.  this keeps the two-list strategy
    /// but fixes the index and documents the approach.
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_head = Box::new(ListNode::new(0));
        let mut even_head = Box::new(ListNode::new(0));

        let mut curr_opt = head;
        let mut curr_even = &mut even_head;
        let mut curr_odd = &mut odd_head;
        let mut idx = 1;

        while let Some(mut curr) = curr_opt {
            let next = curr.next.take();
            if idx % 2 == 1 {
                curr_odd.next = Some(curr);
                curr_odd = curr_odd.next.as_mut().unwrap();
            } else {
                curr_even.next = Some(curr);
                curr_even = curr_even.next.as_mut().unwrap();
            }
            idx += 1;
            curr_opt = next;
        }

        // append even sublist after odd
        curr_odd.next = even_head.next;
        odd_head.next
    }
}

impl Solution {
    /// simple constant‑space solution using two dummy heads.  we iterate the
    /// original list once, peeling nodes off alternately and appending them to
    /// the odd/even chains, then connect the two chains at the end.
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_dummy = Box::new(ListNode::new(0));
        let mut even_dummy = Box::new(ListNode::new(0));

        let mut odd_tail = &mut odd_dummy;
        let mut even_tail = &mut even_dummy;
        let mut take_odd = true;

        while let Some(mut node) = head {
            head = node.next.take();
            if take_odd {
                odd_tail.next = Some(node);
                odd_tail = odd_tail.next.as_mut().unwrap();
            } else {
                even_tail.next = Some(node);
                even_tail = even_tail.next.as_mut().unwrap();
            }
            take_odd = !take_odd;
        }

        odd_tail.next = even_dummy.next;
        odd_dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, Solution, ListNode};

    // helper utilities for building and inspecting lists
    fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        for &v in vals {
            tail.next = Some(Box::new(ListNode::new(v)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }

    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }
        v
    }

    #[test]
    fn draft_works() {
        let list = build_list(&[1, 2, 3, 4, 5]);
        let out = Draft::odd_even_list(list);
        assert_eq!(list_to_vec(out), vec![1, 3, 5, 2, 4]);

        let list = build_list(&[2, 1, 3, 5, 6, 4, 7]);
        let out = Draft::odd_even_list(list);
        assert_eq!(list_to_vec(out), vec![2, 3, 6, 7, 1, 5, 4]);
    }

    #[test]
    fn solution_works() {
        let list = build_list(&[1, 2, 3, 4, 5]);
        assert_eq!(list_to_vec(Solution::odd_even_list(list)), vec![1, 3, 5, 2, 4]);

        let list = build_list(&[2, 1, 3, 5, 6, 4, 7]);
        assert_eq!(list_to_vec(Solution::odd_even_list(list)), vec![2, 3, 6, 7, 1, 5, 4]);

        // some edge cases
        assert_eq!(list_to_vec(Solution::odd_even_list(None)), vec![]);
        assert_eq!(list_to_vec(Solution::odd_even_list(build_list(&[10]))), vec![10]);
        assert_eq!(list_to_vec(Solution::odd_even_list(build_list(&[1,2]))), vec![1,2]);
    }
}
