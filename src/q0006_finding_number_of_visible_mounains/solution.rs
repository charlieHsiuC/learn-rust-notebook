struct Draft;
struct Solution;

impl Draft {
    pub fn is_covered(peak_a: &(i32, i32), peak_b: &(i32, i32)) -> bool {
        let is_swap = peak_a.1 < peak_b.1;

        let mut peak_high = peak_a;
        let mut peak_low = peak_b;
        if is_swap {
            peak_high = peak_b;
            peak_low = peak_a;
        }

        if peak_high.0 == peak_low.0 {
            if peak_high.1 == peak_low.1 {
                return true;
            }
            else {
                return is_swap;
            }
        }

        let x_diff = peak_high.0 - peak_low.0;
        let y_diff = peak_high.1 - peak_low.1;

        if y_diff >= x_diff.abs() {
            return is_swap;
        }
        return false
    }

    pub fn visible_mountains(peaks: Vec<Vec<i32>>) -> i32 {
        let mut cords: Vec<(i32, i32)> = peaks.iter().map(|p| (p[0], p[1])).collect();
        cords.sort_by(|a, b| b.1.cmp(&a.1));
        let mut stk: Vec<(i32, i32, bool)> = Vec::new();

        for cord in cords {
            if stk.is_empty(){
                stk.push((cord.0, cord.1, false));
            }
            else {
                if !stk.iter_mut().any(|e| {
                    let p = (e.0, e.1);
                    if cord == p {
                        e.2 = true;
                        return true;
                    }
                    Self::is_covered(&cord, &p)
                }) {
                    stk.push((cord.0, cord.1, false));
                }
            }
        }
        
        stk.iter().filter(|(_, _, status)| !status).count() as i32
    }
}

impl Solution {
    pub fn visible_mountains(peaks: Vec<Vec<i32>>) -> i32 {
        // Transform coordinates (x, y) into intervals [x-y, x+y].
        // A mountain covers the range [x-y, x+y] on the x-axis.
        let mut intervals: Vec<(i32, i32)> = peaks
            .iter()
            .map(|p| (p[0] - p[1], p[0] + p[1]))
            .collect();

        // Sort intervals:
        // 1. By start point (ascending).
        // 2. If start points are equal, by end point (descending).
        // This ensures that if multiple mountains start at the same point,
        // the widest one (which covers the others) comes first.
        intervals.sort_unstable_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        let mut count = 0;
        let mut max_end = i32::MIN;

        for i in 0..intervals.len() {
            let (start, end) = intervals[i];

            // If the current mountain ends before or at the furthest reach seen so far,
            // it is completely covered by a previous mountain.
            if end <= max_end {
                continue;
            }

            // If it extends further, it *might* be visible.
            // However, we must check for duplicates.
            // Since we sorted by end descending (for same start), duplicates are adjacent.
            // If intervals[i] == intervals[i+1], then intervals[i] is covered by intervals[i+1]
            // (conceptually they cover each other, so neither is visible).
            // Note: We check i + 1 < len to avoid out of bounds.
            let is_duplicate = if i + 1 < intervals.len() {
                intervals[i] == intervals[i+1]
            } else {
                false
            };

            if !is_duplicate {
                count += 1;
            }

            // Update the furthest reach. Even if it's a duplicate (and thus invisible),
            // it physically exists and can cover subsequent mountains.
            max_end = end;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, Solution};

    #[test]
    fn draft_works() {
        assert_eq!(Draft::visible_mountains(vec![vec![2,2], vec![6,3], vec![5,4]]), 2);
        assert_eq!(Draft::visible_mountains(vec![vec![3,1], vec![3,1]]), 0);
        assert_eq!(Draft::visible_mountains(vec![vec![38,26], vec![3,32], vec![2,1]]), 2);
    }

    #[test]
    fn solution_works() {
        assert_eq!(Solution::visible_mountains(vec![vec![2,2], vec![6,3], vec![5,4]]), 2);
        assert_eq!(Solution::visible_mountains(vec![vec![3,1], vec![3,1]]), 0);
        assert_eq!(Solution::visible_mountains(vec![vec![38,26], vec![3,32], vec![2,1]]), 2);
    }
}