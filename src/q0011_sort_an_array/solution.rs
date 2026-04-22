struct Draft;
struct Solution;

impl Draft {
    fn sort_helper(nums: &mut Vec<i32>, l: usize, r: usize) {
        if r - l < 2 {
            return;
        }

        let m: usize = l + (r - l) / 2;
        Self::sort_helper(nums, l, m);
        Self::sort_helper(nums, m, r);

        let mut merged = vec![];
        let mut i = l;
        let mut j = m;

        while i < m && j < r {
            if nums[i] < nums[j] {
                merged.push(nums[i]);
                i += 1;
            } else {
                merged.push(nums[j]);
                j += 1;
            }
        }

        while i < m {
            merged.push(nums[i]);
            i += 1;
        }

        while j < r {
            merged.push(nums[j]);
            j += 1;
        }

        for i in l..r {
            nums[i] = merged[i - l];
        }
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = Vec::<i32>::from(nums);
        let len = arr.len();
        Self::sort_helper(&mut arr, 0, len);
        arr
    }
}

impl Solution {
    /// Iterative sift-down (max-heap). Maintains the heap property for the
    /// subtree rooted at `root` within `nums[0..end]`.
    /// Iterative variant → O(1) stack space (no recursion).
    fn sift_down(nums: &mut [i32], mut root: usize, end: usize) {
        loop {
            let left = 2 * root + 1;
            if left >= end {
                break;
            }
            // Pick the larger child
            let child = if left + 1 < end && nums[left + 1] > nums[left] {
                left + 1
            } else {
                left
            };
            if nums[root] >= nums[child] {
                break;
            }
            nums.swap(root, child);
            root = child;
        }
    }

    /// Heap sort — O(n log n) time, O(1) auxiliary space.
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = nums;
        let n = arr.len();
        if n < 2 {
            return arr;
        }

        // Phase 1: Build a max-heap (bottom-up, O(n))
        for i in (0..n / 2).rev() {
            Self::sift_down(&mut arr, i, n);
        }

        // Phase 2: Repeatedly extract the maximum to the end
        for end in (1..n).rev() {
            arr.swap(0, end);
            Self::sift_down(&mut arr, 0, end);
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, Solution};

    #[test]
    fn draft_works() {
        assert_eq!(Draft::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(
            Draft::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }

    #[test]
    fn solution_works() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }
}
