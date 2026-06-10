struct Draft;
struct Solution;

impl Draft {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i: i32 = m - 1;
        let mut j: i32 = n - 1;
        let len = nums1.len();

        for k in 0..len {
            if i >= 0 && j >= 0 {
                if nums1[i as usize] > nums2[j as usize] {
                    nums1[len - k - 1] = nums1[i as usize];
                    i -= 1;
                } else {
                    nums1[len - k - 1] = nums2[j as usize];
                    j -= 1;
                }
            } else {
                if i >= 0 {
                    nums1[len - k - 1] = nums1[i as usize];
                    i -= 1;
                } else {
                    nums1[len - k - 1] = nums2[j as usize];
                    j -= 1;
                }
            }
        }
    }
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
        let (mut i, mut j, mut write) = (m as usize, n as usize, m as usize + n as usize);

        while write > 0 {
            write -= 1;
            if j > 0 && (i == 0 || nums2[j - 1] > nums1[i - 1]) {
                nums1[write] = nums2[j - 1];
                j -= 1;
            } else {
                nums1[write] = nums1[i - 1];
                i -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Draft, Solution};

    #[test]
    fn draft_works() {
        let mut n1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut n2: Vec<i32> = vec![2, 5, 6];
        Draft::merge(&mut n1, 3, &mut n2, 3);
        assert_eq!(n1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn solution_examples_work() {
        let mut n1 = vec![1, 2, 3, 0, 0, 0];
        let n2 = vec![2, 5, 6];
        Solution::merge(&mut n1, 3, &n2, 3);
        assert_eq!(n1, vec![1, 2, 2, 3, 5, 6]);

        let mut n1 = vec![1];
        let n2: Vec<i32> = vec![];
        Solution::merge(&mut n1, 1, &n2, 0);
        assert_eq!(n1, vec![1]);

        let mut n1 = vec![0];
        let n2 = vec![1];
        Solution::merge(&mut n1, 0, &n2, 1);
        assert_eq!(n1, vec![1]);
    }
}
