# Review: Reverse Linked List

## 1. The Rustacean Way

Refined the code to use the standard iterative pointer manipulation pattern.

```rust
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next.take(); // 1. Save next
        node.next = prev;           // 2. Reverse link
        prev = Some(node);          // 3. Advance prev
    }
    prev
}
```

## 2. Algorithm & Complexity

* **Time:** O(N) - Single pass.
* **Space:** O(1) - In-place pointer rewiring.

## 3. Key Rust Concept

* **`Option::take()`**: Crucial for linked lists. It allows taking ownership of a value inside an `Option` (like `node.next`) leaving `None` in its place. This prevents "use after move" errors when we need to modify the struct we are iterating over.
* **`while let`**: The idiomatic loop for consuming `Option` based structures.

## 4. Comparison

The Draft version manually unrolled the first iteration (handling `head` separately). The Solution version unifies the logic by initializing `prev` to `None`, making the code cleaner and handling empty lists implicitly.
