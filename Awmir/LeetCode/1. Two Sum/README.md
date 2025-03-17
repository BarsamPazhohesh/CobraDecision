# Two Sum - Problem Statement & Test Cases
### [Question Link](https://leetcode.com/problems/two-sum/description/)
## **Problem Statement**

Given an array of integers `nums` and an integer `target`, return **indices** of the two numbers such that they add up to `target`.

- You may assume that each input would have **exactly one solution**, and you **may not use the same element twice**.
- You can return the answer in **any order**.

---

## **Constraints**
- `2 <= nums.length <= 10^4`
- `-10^9 <= nums[i] <= 10^9`
- `-10^9 <= target <= 10^9`
- **Only one valid answer exists.**

---

## **Test Cases**

| **Test Case**       | **Input `nums`**               | **Input `target`** | **Expected Output** |
|---------------------|--------------------------------|---------------------|----------------------|
| **Example 1**       | `[2,7,11,15]`                 | `9`                 | `[0,1]`             |
| **Example 2**       | `[3,2,4]`                     | `6`                 | `[1,2]`             |
| **Example 3**       | `[3,3]`                       | `6`                 | `[0,1]`             |
| **Edge Case 1**     | `[1,5]`                       | `6`                 | `[0,1]`             |
| **Edge Case 2**     | `[-3,4,3,90]`                 | `0`                 | `[0,2]`             |
| **Edge Case 3**     | `[1000000000, -1000000000, 3, 7]` | `0`              | `[0,1]`             |
| **Edge Case 4**     | `[10, 25, 5, 15]`             | `20`                | `[2,3]`             |
| **Edge Case 5**     | `[1, 2, 3, 4, 5, 10]`         | `15`                | `[4,5]`             |
| **Edge Case 6**     | `[8, 12, 3, 7]`               | `20`                | `[0,1]`             |
| **Edge Case 7**     | `[1, 2, 3, 4, 5, 6, 7]`       | `13`                | `[5,6]`             |

This table provides a structured set of test cases covering basic, edge, and special scenarios.
