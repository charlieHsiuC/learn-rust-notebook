use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Draft;
struct Solution;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Draft {
    fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.len() != inorder.len() || inorder.len() == 0 {
            return None;
        }

        if inorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(inorder[0]))));
        }

        let p_last = postorder.last().unwrap();
        let p_len = postorder.len();
        let i_pos = inorder.iter().position(|x| x == p_last).unwrap();

        let mut node = TreeNode::new(*p_last);
        node.left = Self::helper(&inorder[0..i_pos], &postorder[0..i_pos]);
        node.right = Self::helper(&inorder[i_pos + 1..], &postorder[i_pos..p_len - 1]);

        Some(Rc::new(RefCell::new(node)))
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::helper(&inorder, &postorder);
    }
}

impl Solution {
    fn build(
        in_left: usize,
        in_right: usize,
        postorder: &[i32],
        post_idx: &mut isize,
        inorder_index: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_left >= in_right {
            return None;
        }

        let root_val = postorder[*post_idx as usize];
        *post_idx -= 1;

        let split = inorder_index[&root_val];
        let mut root = TreeNode::new(root_val);

        root.right = Self::build(split + 1, in_right, postorder, post_idx, inorder_index);
        root.left = Self::build(in_left, split, postorder, post_idx, inorder_index);

        Some(Rc::new(RefCell::new(root)))
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() != postorder.len() || inorder.is_empty() {
            return None;
        }

        let inorder_index: HashMap<i32, usize> = inorder
            .iter()
            .enumerate()
            .map(|(idx, &value)| (value, idx))
            .collect();
        let mut post_idx = postorder.len() as isize - 1;

        Self::build(0, inorder.len(), &postorder, &mut post_idx, &inorder_index)
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn inorder_values(root: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            inorder_values(&node.left, out);
            out.push(node.val);
            inorder_values(&node.right, out);
        }
    }

    fn postorder_values(root: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            postorder_values(&node.left, out);
            postorder_values(&node.right, out);
            out.push(node.val);
        }
    }

    fn assert_matches_traversals(
        root: Option<Rc<RefCell<TreeNode>>>,
        inorder: &[i32],
        postorder: &[i32],
    ) {
        let mut built_inorder = Vec::new();
        let mut built_postorder = Vec::new();

        inorder_values(&root, &mut built_inorder);
        postorder_values(&root, &mut built_postorder);

        assert_eq!(built_inorder, inorder);
        assert_eq!(built_postorder, postorder);
    }

    #[test]
    fn draft_works() {
        let inorder = [9, 3, 15, 20, 7];
        let postorder = [9, 15, 7, 20, 3];
        let tree = Draft::build_tree(inorder.to_vec(), postorder.to_vec());
        assert_matches_traversals(tree, &inorder, &postorder);
    }

    #[test]
    fn solution_works() {
        let inorder = [9, 3, 15, 20, 7];
        let postorder = [9, 15, 7, 20, 3];
        let tree = Solution::build_tree(inorder.to_vec(), postorder.to_vec());
        assert_matches_traversals(tree, &inorder, &postorder);

        let inorder = [-1];
        let postorder = [-1];
        let tree = Solution::build_tree(inorder.to_vec(), postorder.to_vec());
        assert_matches_traversals(tree, &inorder, &postorder);

        let inorder = [1, 2, 3, 4];
        let postorder = [1, 2, 3, 4];
        let tree = Solution::build_tree(inorder.to_vec(), postorder.to_vec());
        assert_matches_traversals(tree, &inorder, &postorder);
    }
}
