pub fn two_sum_sorted(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let sum = numbers[left] + numbers[right];

        if sum == target {
            return vec![(left + 1) as i32, (right + 1) as i32];
        }

        if sum > target { 
            right -= 1;
        } else {
            left += 1;
        }
    }

    vec![] 
}

#[cfg(test)]
mod two_sum_sorted_tests {
    use super::*;

    #[test]
    fn returns_correct_indices_for_basic_case() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum_sorted(numbers, target), vec![1, 2]); // 2 + 7 = 9
    }

    #[test]
    fn returns_correct_indices_for_three_elements() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        assert_eq!(two_sum_sorted(numbers, target), vec![1, 3]); // 2 + 4 = 6
    }

    #[test]
    fn handles_negative_numbers_correctly() {
        let numbers = vec![-1, 0];
        let target = -1;
        assert_eq!(two_sum_sorted(numbers, target), vec![1, 2]); // -1 + 0 = -1
    }

    #[test]
    fn finds_correct_pair_in_large_sorted_list() {
        let numbers = vec![1, 2, 3, 10, 20, 30, 40, 50];
        let target = 23;
        assert_eq!(two_sum_sorted(numbers, target), vec![3, 5]); // 3 + 20 = 23
    }

    #[test]
    fn works_with_distinct_values() {
        let numbers = vec![1, 10, 15, 20, 25];
        let target = 30;
        assert_eq!(two_sum_sorted(numbers, target), vec![2, 4]); // 10 + 20 = 30
    }
}
