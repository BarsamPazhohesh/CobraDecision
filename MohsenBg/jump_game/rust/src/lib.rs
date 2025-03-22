use std::cmp::max;

pub fn greedy_solution(nums: Vec<i32>) -> bool {
    let mut max_reach: i32 = 0;

    for (index, &num) in nums.iter().enumerate() {
        if index as i32 > max_reach {
            return false;
        }
        max_reach = max(max_reach, index as i32 + num);

        if max_reach >= (nums.len() as i32 - 1) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod jump_game_tests {
    use super::*;

    #[test]
    fn can_reach_with_multiple_paths() {
        assert_eq!(greedy_solution(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn blocked_by_zero_cannot_reach_end() {
        assert_eq!(greedy_solution(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn single_element_already_at_end() {
        assert_eq!(greedy_solution(vec![0]), true);
    }

    #[test]
    fn stuck_at_second_position() {
        assert_eq!(greedy_solution(vec![1, 0, 1]), false);
    }

    #[test]
    fn can_jump_directly_to_end() {
        assert_eq!(greedy_solution(vec![2, 0, 0]), true);
    }

    #[test]
    fn can_reach_before_zero_block() {
        assert_eq!(greedy_solution(vec![1, 2, 3, 4, 5, 0, 0, 0, 0]), true);
    }

    #[test]
    fn decreasing_jumps_lead_to_stuck() {
        assert_eq!(greedy_solution(vec![5, 4, 3, 2, 1, 0, 0]), false);
    }

    #[test]
    fn large_initial_jump_reaches_end() {
        assert_eq!(
            greedy_solution(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
            true
        );
    }

    #[test]
    fn small_but_sufficient_jumps() {
        assert_eq!(greedy_solution(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), true);
    }

    #[test]
    fn blocked_in_middle_cannot_reach_end() {
        assert_eq!(greedy_solution(vec![1, 1, 1, 0, 1, 1, 1, 1, 1]), false);
    }
}
