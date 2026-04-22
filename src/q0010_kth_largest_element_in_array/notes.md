# Q0010 - Kth Largest Element in an Array

## 1. The Rustacean Way

The most idiomatic one-liner leverages `slice::select_nth_unstable`, a standard library method built on introselect — an O(n) worst-case selection algorithm. For a manual approach, the idiomatic style uses 3-way (Dutch National Flag) partitioning over `&mut [i32]` slices, with sub-slice recursion instead of explicit index arithmetic.

**Idiomatic rewrite of the core idea:**

```rust
pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    *nums.select_nth_unstable(n - k as usize).1
}
```

---

## 2. Algorithm & Complexity

### Quickselect (the Draft's pattern)

- **Average**: O(n) time, O(log n) space (call stack)
- **Worst case**: O(n²) — triggered when pivot is always min or max (e.g. sorted input with first-element pivot)
- Random pivot or median-of-3 mitigates this

### 3-way Partition Quickselect (Solution)

- Same asymptotic complexity, but handles arrays with many duplicates in O(n) rather than degrading to O(n²)
- Divides elements into: `> pivot | == pivot | < pivot` in a single pass

### Min-Heap alternative

- **O(n log k)** time, **O(k)** space
- Useful when k << n or streaming input

### `select_nth_unstable` (standard library)

- Introselect: hybrid of quickselect + heapselect
- **O(n) worst case** guaranteed, no extra space
- `select_nth_unstable(idx)` rearranges so `nums[idx]` is in its sorted position; elements on each side are unsorted but correctly partitioned

---

## 3. Blind Spot Detection

### Bug 1 — Misleading pivot swap

```rust
nums.swap(l, j - 1);
```

After this swap, `nums[l]` holds a "greater-than" element and `nums[j-1]` holds the pivot. The rank computation still works, but the invariant (pivot at `l` → partition → pivot at final position) is broken mid-way, making the code hard to audit.

### Bug 2 — Partial duplicate handling

The draft counts duplicates in `count` and uses it for early termination, but then recurses into the right partition **without subtracting** `count - 1` from `k`. This accidentally works because the right partition (`nums[j..r]`) **still contains the unswapped duplicates**, so they consume ranks naturally during the recursion. This is correct but non-obvious — a 3-way partition makes the duplicate logic explicit and eliminates re-processing duplicates.

### Bug 3 — Unnecessary clone

```rust
let mut arr: Vec<i32> = nums.clone();
Self::find_kth_helper(&mut arr, 0, nums.len(), k as usize)
```

`nums` is already owned (moved in by value). You can mutate it directly — no clone needed.

### Bug 4 — `&mut Vec<i32>` vs `&mut [i32]`

Passing `Vec` by mutable reference ties the helper to heap-allocated vectors and requires explicit `l`/`r` index arithmetic. Slices (`&mut [i32]`) enable sub-slice recursion (`&mut nums[..lo]`), removing the need for index bounds entirely and eliminating any usize underflow risk.

### Bug 5 — Non-idiomatic `return` statements

Rust is expression-oriented. Using `return` in the middle of if/else chains (especially as the last expression in each arm) suppresses this: prefer returning the expression directly from the `if`/`else` block.

---

## 4. The Why

| Aspect | Draft | Idiomatic |
|---|---|---|
| Mutation scope | `&mut Vec<i32>` + explicit `l`, `r` | `&mut [i32]` sub-slices |
| Duplicate handling | Implicit via right-recursion | Explicit 3-way partition |
| Pivot management | Swap to position `j-1` | In-place Dutch National Flag |
| Worst case | O(n²) on sorted input | O(n) with `select_nth_unstable` |
| usize safety | `j - 1` could theoretically panic | Slice length is the bound |

Slices are the idiomatic unit of contiguous-memory work in Rust. Recursive algorithms on slices (`&mut nums[..lo]`, `&mut nums[mid..]`) let the compiler manage bounds — not you. This removes an entire class of off-by-one errors.

---

## 5. Key Rust Concepts

- `slice::select_nth_unstable` / `select_nth_unstable_by`
- Dutch National Flag 3-way partition
- `&mut [T]` slice recursion (`&mut nums[..lo]`, `&mut nums[mid..]`)
- `std::collections::BinaryHeap` with `std::cmp::Reverse<T>` (min-heap)
- `std::cmp::Ordering` with exhaustive `match`
- Expression-oriented returns (no trailing `return`)
