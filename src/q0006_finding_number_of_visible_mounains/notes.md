# Review of `q0006_finding_number_of_visible_mounains`

Here is a comprehensive review of your initial draft for the "Finding the Number of Visible Mountains" problem.

## 1. The Rustacean Way: Idiomatic Refinements

- **Data Transformation:** Your draft operates directly on the raw coordinates `(x, y)`. In Rust, it's idiomatic to transform data into a shape that makes the problem easier to solve. Converting the mountains into intervals `[start, end]` allows us to use standard sorting and iteration patterns.
- **Sorting with Custom Comparators:** You used `sort_by` to sort by height. The idiomatic way to handle complex ordering in Rust is `sort_unstable_by` (faster than stable sort when stability isn't needed) with a closure that defines the exact precedence rules (e.g., start point ascending, then end point descending).
- **Avoid `Vec<Vec<i32>>` overhead:** While the input is `Vec<Vec<i32>>`, converting this immediately to a `Vec<(i32, i32)>` (or a custom struct) is better than indexing `p[0]`, `p[1]` repeatedly.

## 2. Algorithm & Complexity

- **Your Draft:**
  - **Algorithm:** Brute Force / Pairwise Comparison. You sort by height and then check each mountain against a list of "accepted" mountains.
  - **Time Complexity:** $O(N^2)$ in the worst case. If no mountain covers another, you might end up comparing the $i$-th mountain against all previous $i-1$ mountains. Given $N \le 10^5$, this will likely result in a **Time Limit Exceeded (TLE)**.
  - **Space Complexity:** $O(N)$ to store the stack.

- **Optimal Approach:**
  - **Algorithm:** Coordinate Transformation + Greedy (Sweep Line).
    - A mountain at $(x, y)$ covers the interval $[x-y, x+y]$ on the x-axis.
    - Problem reduces to: "How many intervals are not contained within any other interval?"
    - Sort intervals by `start` ascending. If `start` is equal, sort by `end` descending.
    - Iterate once. If an interval ends before the current `max_end`, it is covered.
  - **Time Complexity:** $O(N \log N)$ due to sorting. The scan is $O(N)$.
  - **Space Complexity:** $O(N)$ to store the transformed intervals.

## 3. Blind Spot Detection

- **Geometry Logic:** Your `is_covered` function attempts to check coverage using raw coordinates. While mathematically derivable, it's complex to get right with edge cases. The interval transformation $[x-y, x+y]$ simplifies the logic significantly: interval $A$ covers $B$ iff $start_A \le start_B$ and $end_A \ge end_B$.
- **Duplicate Handling:** The problem states a mountain is visible if it's not within *another* mountain. If two mountains are identical (same $x$, same $y$), they cover each other. Therefore, **neither** is visible. Your draft attempts to handle this with the `status` flag, but the $O(N^2)$ logic makes it tricky. In the sorted interval approach, duplicates appear adjacent to each other and are easy to filter.

## 4. The "Why": Benefits of the Idiomatic Approach

1. **Performance:** The shift from $O(N^2)$ to $O(N \log N)$ is critical for passing the test cases.
2. **Readability:** Transforming the problem into "Interval Scheduling" or "Interval Covering" makes the logic standard and easier to reason about than custom geometry checks.
3. **Robustness:** Sorting by `start` asc and `end` desc handles the "containment" logic naturally. If $A$ starts before $B$ (or at the same time but ends later), $A$ is processed first. We then only need to check if $B$ ends before $A$ ends to know if $B$ is covered.

## 5. Key Rust Concept

- **`sort_unstable_by`:** A high-performance sorting method in Rust. It does not preserve the order of equal elements (unstable), which allows it to be faster and use less memory than `sort_by`.
- **Custom Ordering:**

  ```rust
  intervals.sort_unstable_by(|a, b| {
      if a.0 != b.0 {
          a.0.cmp(&b.0) // Primary: Start time ascending
      } else {
          b.1.cmp(&a.1) // Secondary: End time descending
      }
  });
  ```

  This pattern is essential for interval problems to ensure the "largest covering" interval appears first when start times match.
