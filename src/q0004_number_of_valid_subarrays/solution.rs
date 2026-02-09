pub fn valid_subarrays(nums: &[i32]) -> i32 {
    let mut total_valid_subarrays = 0;
    // The stack stores tuples of (value, count_of_valid_subarrays_starting_with_this_value).
    // It maintains a monotonically increasing order of values from bottom to top.
    let mut stack: Vec<(i32, i32)> = Vec::new();

    // We iterate from right to left.
    for &num in nums.iter().rev() {
        // This will be the count of valid subarrays starting with `num`.
        // It's at least 1 (the subarray `[num]` itself).
        let mut count_for_this_num = 1;

        // While the stack is not empty and the current number is smaller or equal to
        // the number at the top of the stack...
        while let Some(&(top_val, top_count)) = stack.last() {
            if num <= top_val {
                // The element `num` is smaller, so it can form valid subarrays with everything
                // `top_val` could, plus `top_val` itself. We absorb `top_val`'s count.
                stack.pop();
                count_for_this_num += top_count;
            } else {
                // We found a number on the stack smaller than `num`.
                // `num` cannot form a valid subarray that extends past this smaller number.
                break;
            }
        }
        stack.push((num, count_for_this_num));
        total_valid_subarrays += count_for_this_num;
    }

    total_valid_subarrays
}

#[cfg(test)]
mod tests {
    use super::valid_subarrays;

    #[test]
    fn it_works() {
        assert_eq!(valid_subarrays(&[1, 4, 2, 5, 3]), 11);
        assert_eq!(valid_subarrays(&[2, 2, 2]), 6);
        assert_eq!(valid_subarrays(&[3, 2, 1]), 3);
        assert_eq!(valid_subarrays(&[14,20,5,13,18,8,2,14,3,13]), 19);
    }
}