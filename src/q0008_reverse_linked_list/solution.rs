struct Draft;
struct Solution;

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
}

impl Draft {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                let mut next_opt = node.next.take();
                while let Some(mut next) = next_opt {
                    let tmp = next.next.take();
                    next.next = Some(node);
                    node = next;
                    next_opt = tmp;
                }

                Some(node)
            }
            None => None,
        }
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take(); // Save next node and detach connection
            node.next = prev; // Point current node backwards
            prev = Some(node); // Move prev to current
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, ListNode, Solution};

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
        let out = Draft::reverse_list(list);
        assert_eq!(list_to_vec(out), vec![5, 4, 3, 2, 1]);

        let list = build_list(&[1, 2]);
        let out = Draft::reverse_list(list);
        assert_eq!(list_to_vec(out), vec![2, 1]);

        let list = build_list(&[]);
        let out = Draft::reverse_list(list);
        assert_eq!(list_to_vec(out), vec![]);
    }

    #[test]
    fn solution_works() {
        let list = build_list(&[1, 2, 3, 4, 5]);
        let out = Solution::reverse_list(list);
        assert_eq!(list_to_vec(out), vec![5, 4, 3, 2, 1]);

        let list = build_list(&[1, 2]);
        let out = Solution::reverse_list(list);
        assert_eq!(list_to_vec(out), vec![2, 1]);

        let list = build_list(&[]);
        let out = Solution::reverse_list(list);
        assert_eq!(list_to_vec(out), vec![]);
    }
}
