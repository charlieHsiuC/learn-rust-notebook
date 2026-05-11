# Q0013 Construct Binary Tree from Inorder and Postorder Traversal - Review Notes

## 1. The Rustacean Way

Your draft gets the core recursion right: take the last postorder value as the
root, split inorder around it, and recursively build left/right subtrees.
That is the correct reconstruction pattern.

The main thing holding it back is not correctness but how much work each
recursive call repeats:

- `inorder.iter().position(...)` scans linearly every time.
- Re-slicing `inorder` and `postorder` keeps the code simple, but it also hides
 the subtree boundaries that matter for performance.
- `unwrap()` is logically safe under valid constraints, but it pushes an
 important invariant out of the type system and into problem assumptions.

An idiomatic optimized version in Rust usually looks like this:

```rust
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
  root.right = build(split + 1, in_right, postorder, post_idx, inorder_index);
  root.left = build(in_left, split, postorder, post_idx, inorder_index);

  Some(Rc::new(RefCell::new(root)))
}
```

Why this is more Rustacean:

- It passes slices by reference instead of cloning or reallocating.
- It tracks subtree ranges with indices, which makes ownership simple and costs
 predictable.
- It preprocesses a `HashMap` once, so recursive calls stay cheap.
- It encodes the recursive state explicitly: current inorder window plus current
 postorder cursor.

## 2. Algorithm & Complexity

Pattern: **Tree reconstruction from traversal pairs**.

Key facts:

- In postorder, the last value is always the current subtree root.
- In inorder, values left of the root belong to the left subtree, and values
 right of the root belong to the right subtree.
- Because we consume postorder from the end, we must build the **right subtree
 before the left subtree**.

Draft complexity:

- **Time:** O(n^2) worst case.
 Every recursive call does a linear `position` scan, and for a skewed tree that
 becomes `n + (n - 1) + ... + 1`.
- **Extra Space:** O(n) recursion stack in the worst case, plus temporary slice
 views.

Optimized solution complexity:

- **Time:** O(n)
 Each node is processed once, and each root lookup is O(1) average via
 `HashMap`.
- **Extra Space:** O(n)
 For the index map plus recursion stack in the worst case.

## 3. Blind Spot Detection

Things easy to miss in this problem:

- **Right-before-left recursion order is mandatory.**
 If you build left first while consuming postorder backwards, you will assign
 nodes to the wrong subtree.
- **The draft assumes valid inputs and uniqueness.**
 That matches the problem constraints, so it is acceptable, but the `unwrap()`
 on root lookup would panic immediately if those assumptions were violated.
- **Skewed trees are the real performance trap.**
 Balanced examples often make the draft look fast enough, but the worst case is
 where the repeated root search becomes expensive.
- **Single-node and fully skewed trees should be tested explicitly.**
 They expose off-by-one mistakes in index ranges much faster than balanced
 examples.

## 4. The Why

The idiomatic Rust version is not just cleaner. It is more robust because the
state transitions are explicit:

- `in_left..in_right` clearly describes the current subtree.
- `post_idx` makes the consumption order visible.
- The `HashMap` removes repeated work instead of hoping input sizes stay small.

It is also potentially faster for two concrete reasons:

- It avoids repeated linear searches through `inorder`.
- It keeps recursion state minimal: a few indices and shared references, rather
 than rebuilding logical partitions by searching every time.

This is the general lesson: in Rust, “more explicit state” often means both
better performance and code that is easier to reason about, because ownership,
borrowing, and invariants become visible instead of implicit.

## 5. Key Rust Concepts

- `HashMap` preprocessing for O(1) lookup
- Recursive helpers over `&[T]` plus index ranges
- Mutating shared recursion state with `&mut isize`
- `Rc<RefCell<TreeNode>>` for LeetCode-style shared tree nodes
- Recursion order driven by traversal semantics
