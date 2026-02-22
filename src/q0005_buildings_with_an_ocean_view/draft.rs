pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
    let mut max_height_from_right = 0;
    let mut building_with_view: Vec<i32> = Vec::new();

    for (i, &height) in heights.iter().enumerate().rev() {
        if height > max_height_from_right {
            max_height_from_right = height;
            building_with_view.push(i as i32);
        }
    }

    building_with_view.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::find_buildings;

    #[test]
    fn it_works() {
        assert_eq!(find_buildings(vec![4, 2, 3, 1]), vec![0, 2, 3]);
        assert_eq!(find_buildings(vec![4, 3, 2, 1]), vec![0, 1, 2, 3]);
        assert_eq!(find_buildings(vec![1, 3, 2, 4]), vec![3]);
    }
}