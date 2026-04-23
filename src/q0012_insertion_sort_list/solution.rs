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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(0));

        let mut next = head;
        while let Some(mut node) = next {
            next = node.next.take();
            let mut prev = &mut root.next;

            while prev.as_ref().is_some_and(|curr| curr.val <= node.val) {
                prev = &mut prev.as_mut().unwrap().next;
            }

            node.next = prev.take();
            *prev = Some(node);
        }

        root.next
    }
}

impl Solution {
    fn insert_sorted(sorted: &mut Option<Box<ListNode>>, mut node: Box<ListNode>) {
        let mut cursor = sorted;

        loop {
            match cursor {
                Some(curr) if curr.val <= node.val => {
                    cursor = &mut curr.next;
                }
                _ => {
                    node.next = cursor.take();
                    *cursor = Some(node);
                    break;
                }
            }
        }
    }

    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sorted = None;

        while let Some(mut node) = head {
            head = node.next.take();
            Self::insert_sorted(&mut sorted, node);
        }

        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, ListNode, Solution};

    fn from_vec(values: &[i32]) -> Option<Box<ListNode>> {
        values
            .iter()
            .rev()
            .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
    }

    fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        while let Some(node) = head {
            out.push(node.val);
            head = node.next;
        }
        out
    }

    #[test]
    fn draft_works() {
        let head = from_vec(&[4, 2, 1, 3]);
        assert_eq!(to_vec(Draft::insertion_sort_list(head)), vec![1, 2, 3, 4]);

        let head = from_vec(&[-1, 5, 3, 4, 0]);
        assert_eq!(
            to_vec(Draft::insertion_sort_list(head)),
            vec![-1, 0, 3, 4, 5]
        );
    }

    #[test]
    fn solution_works() {
        let head = from_vec(&[4, 2, 1, 3]);
        assert_eq!(
            to_vec(Solution::insertion_sort_list(head)),
            vec![1, 2, 3, 4]
        );

        let head = from_vec(&[-1, 5, 3, 4, 0]);
        assert_eq!(
            to_vec(Solution::insertion_sort_list(head)),
            vec![-1, 0, 3, 4, 5]
        );

        let head = from_vec(&[]);
        assert_eq!(
            to_vec(Solution::insertion_sort_list(head)),
            Vec::<i32>::new()
        );

        let head = from_vec(&[1, 1, 1]);
        assert_eq!(to_vec(Solution::insertion_sort_list(head)), vec![1, 1, 1]);
    }
}
