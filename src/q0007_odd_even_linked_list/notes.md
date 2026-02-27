# Review for odd-even linked list problem

## 1. The Rustacean Way

- **Ownership & `Option::take`**: the idiomatic solution manipulates the list in-place
  by taking `next` links out of nodes instead of building new ones. this avoids
  allocations and keeps the algorithm O(1) space.
- **Mutable borrowing**: using `as_mut()` to get `&mut` references to nodes allows
  us to walk and update the list without `unsafe` or cloning.
- **Pattern matching**: the loop uses `while let Some(e_node) = even` and
  `if let Some(mut next_odd) = e_node.next.take()` to concisely handle `Option`
  cases.
- **Helper functions in tests** `build_list`/`list_to_vec` reduce repetition and
  make the tests readable.

## 2. Algorithm & Complexity

The algorithm maintains two running pointers, `odd` and `even` (with a separate
`even_head` to remember the start of the even sublist). In each iteration we
splice the next node onto the odd chain, advance both pointers by two steps, and
finally append the even chain to the end of the odd chain.

- **Time:** O(n) — each node is visited a constant number of times.
- **Space:** O(1) extra — pointers are reused and no new `ListNode`s are
  allocated (aside from the dummy in the draft, which is constant).

## 3. Blind Spot Detection

- The original draft used a counter starting at 0, which meant the *first*
  element was treated as even. This produced the reverse order of what the
  prompt asked for. I corrected it by switching to 1-based indexing in the
  draft and explained the mistake.
- Edge cases such as an empty list or a one‑node list must be handled; the
  solution early-returns when `head` is `None`.
- No allocation requirement was stated explicitly, but the LeetCode constraint
  of O(1) extra space implies you should not build new nodes for each element.

## 4. The Why

The idiomatic version is cleaner because it operates directly on the input list
with minimal temporary state. By relying on `Option::take()` and mutable
references, we avoid fiddly pointer juggling and extra dummy nodes. The code is
also arguably more performant: it only rearranges links and doesn't allocate or
clone any data.

## 5. Key Rust Concept Learned

- **Option::take** for moving values out of `Option` without cloning
- **Mutable borrowing** (`as_mut`) to traverse and modify linked structures
- **Linked-list pointer re‑wiring** — a good exercise in ownership and `Option`
