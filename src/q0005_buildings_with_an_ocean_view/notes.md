# Review of `q0005_buildings_with_an_ocean_view`

Here is a comprehensive review of your initial draft for the "Buildings With an Ocean View" problem.

## 1. The Rustacean Way: Idiomatic Refinements

- **Input Type (`&[i32]` vs `Vec<i32>`):** Your draft takes `Vec<i32>` by value. This forces the caller to give up ownership of the vector. Changing the signature to `&[i32]` allows the function to accept references to vectors, arrays, or slices, which is much more flexible and idiomatic for read-only operations.
- **In-Place Reversal:** Instead of `into_iter().rev().collect()`, which consumes the vector and builds a new one, you can use `result.reverse()` to reverse the vector in place. This is slightly more efficient as it avoids allocation overhead for a new vector.

## 2. Algorithm & Complexity

- **Algorithm:** Linear Scan (Right-to-Left).
  - You iterate from the rightmost building to the left.
  - You maintain a `max_height` variable representing the tallest building seen so far to the right.
  - If the current building is taller than `max_height`, it has an ocean view.
- **Time Complexity:** `O(N)`, where `N` is the number of buildings. We traverse the list once.
- **Space Complexity:** `O(1)` auxiliary space (excluding the space required for the output vector).

## 3. Blind Spot Detection

- **The "Height 0" Edge Case:** Your draft initializes `max_height_from_right = 0`.
  - **Scenario:** Consider `heights = [0]`. The loop runs for `height = 0`. The condition `0 > 0` is false. The result is `[]`.
  - **Correct Behavior:** The last building always sees the ocean, even if its height is 0. The result should be `[0]`.
  - **Fix:** Initialize `max_height` to `-1` (assuming heights are non-negative) or `i32::MIN`. This ensures the first building encountered from the right (the last one in the list) is always added.

## 4. The "Why": Benefits of the Idiomatic Approach

1. **Flexibility:** Using `&[i32]` makes your function usable in more contexts (e.g., with a static array or a slice of a larger vector) without needing `.to_vec()` or `.clone()`.
2. **Correctness:** Handling the initialization of `max_height` correctly ensures the solution works for all valid inputs, including edge cases with zero-height buildings.
3. **Performance:** Avoiding unnecessary allocations (via `&[i32]` and in-place `reverse`) makes the code faster and lighter on memory.

## 5. Key Rust Concept

- **Slices (`&[T]`):** A view into a contiguous sequence of elements in a collection. Slices are a fundamental tool in Rust for writing generic, efficient functions that operate on arrays and vectors.
- **`enumerate().rev()`:** A common pattern to iterate backwards while keeping the original indices. Note that `rev()` reverses the iterator, so `enumerate()` must be called *before* `rev()` if you want indices `0..N` to be associated with the elements correctly, but yielded in reverse order `(N-1, item), (N-2, item)...`.
