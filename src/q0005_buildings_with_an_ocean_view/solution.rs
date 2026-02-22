pub fn find_buildings(heights: &[i32]) -> Vec<i32> {
    let mut max_height = -1;
    let mut result = Vec::new();

    // Iterate backwards from the ocean side (right to left).
    // We use enumerate().rev() to get the original indices.
    for (i, &height) in heights.iter().enumerate().rev() {
        // If the current building is taller than the max height seen from the right,
        // it has an ocean view.
        if height > max_height {
            result.push(i as i32);
            max_height = height;
        }
    }

    // The result was collected in reverse order (right-to-left), so we reverse it back
    // to return indices in increasing order.
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::find_buildings;

    #[test]
    fn it_works() {
        assert_eq!(find_buildings(&[4, 2, 3, 1]), vec![0, 2, 3]);
        assert_eq!(find_buildings(&[4, 3, 2, 1]), vec![0, 1, 2, 3]);
        assert_eq!(find_buildings(&[1, 3, 2, 4]), vec![3]);
        assert_eq!(find_buildings(&[2, 2, 2, 2]), vec![3]); // Only the last one sees the ocean
        assert_eq!(find_buildings(&[0]), vec![0]); // Height 0 edge case
    }
}