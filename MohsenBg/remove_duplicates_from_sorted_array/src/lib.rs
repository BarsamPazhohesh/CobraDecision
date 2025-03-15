use std::num;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut unique_index = 1;

    for index in 1..nums.len() {
        if nums[index] != nums[index - 1] {
            nums[unique_index] = nums[index];
            unique_index += 1;
        }
    }

    unique_index as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_with_basic_input() {
        //  Test case 1: Standard test case with duplicates
        let mut nums = vec![1, 1, 2];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(nums[..k as usize], vec![1, 2]);
    }

    #[test]
    fn test_remove_duplicates_with_multiple_duplicates() {
        // Test case 2: Test with multiple duplicates
        let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k2 = remove_duplicates(&mut nums2);
        assert_eq!(k2, 5);
        assert_eq!(nums2[..k2 as usize], vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_duplicates_with_single_element() {
        // Test case 3: Test with a single element
        let mut nums3 = vec![1];
        let k3 = remove_duplicates(&mut nums3);
        assert_eq!(k3, 1);
        assert_eq!(nums3[..k3 as usize], vec![1]);
    }

    #[test]
    fn test_remove_duplicates_with_no_duplicates() {
        // Test case 4: Test with no duplicates
        let mut nums4 = vec![1, 2, 3, 4];
        let k4 = remove_duplicates(&mut nums4);
        assert_eq!(k4, 4);
        assert_eq!(nums4[..k4 as usize], vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_duplicates_with_all_identical_elements() {
        // Test case 5: Test with all duplicates
        let mut nums5 = vec![5, 5, 5, 5, 5];
        let k5 = remove_duplicates(&mut nums5);
        assert_eq!(k5, 1);
        assert_eq!(nums5[..k5 as usize], vec![5]);
    }

    #[test]
    fn test_remove_duplicates_with_negative_numbers() {
        // Test case 6: Test with negative numbers
        let mut nums6 = vec![-1, -1, 0, 0, 1, 1];
        let k6 = remove_duplicates(&mut nums6);
        assert_eq!(k6, 3);
        assert_eq!(nums6[..k6 as usize], vec![-1, 0, 1]);
    }

    #[test]
    fn test_remove_duplicates_with_large_input() {
        // Test case 7: Test with very large array
        let mut nums7 = (0..10000).collect::<Vec<i32>>(); // sorted array from 0 to 9999
        let k7 = remove_duplicates(&mut nums7);
        assert_eq!(k7, 10000);
        assert_eq!(nums7[..k7 as usize], (0..10000).collect::<Vec<i32>>());
    }

    #[test]
    fn test_remove_duplicates_with_all_elements_identical() {
        // Test case 8: Test with array of identical elements
        let mut nums8 = vec![2; 10000];
        let k8 = remove_duplicates(&mut nums8);
        assert_eq!(k8, 1);
        assert_eq!(nums8[..k8 as usize], vec![2]);
    }

    #[test]
    fn test_remove_duplicates_with_mixed_positive_and_negative_elements() {
        // Test case 9: Test with a mix of positive and negative elements
        let mut nums9 = vec![-1, -1, 0, 1, 1, 2];
        let k9 = remove_duplicates(&mut nums9);
        assert_eq!(k9, 4);
        assert_eq!(nums9[..k9 as usize], vec![-1, 0, 1, 2]);
    }

    #[test]
    fn test_remove_duplicates_with_already_unique_elements() {
        // Test case 10: Test with already unique elements
        let mut nums10 = vec![10, 20, 30, 40, 50];
        let k10 = remove_duplicates(&mut nums10);
        assert_eq!(k10, 5);
        assert_eq!(nums10[..k10 as usize], vec![10, 20, 30, 40, 50]);
    }

    #[test]
    fn test_remove_duplicates_with_empty_array() {
        // Test case 11: Test with an empty array
        let mut nums11: Vec<i32> = Vec::new();
        let k11 = remove_duplicates(&mut nums11);
        assert_eq!(k11, 0);
        assert_eq!(nums11, Vec::<i32>::new());
    }
}
