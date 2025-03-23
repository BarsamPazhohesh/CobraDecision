# Two Sum II - Input Array is Sorted

## Problem Statement

Given a **1-indexed** array of integers `numbers` that is already sorted in **non-decreasing** order, find two numbers such that they add up to a specific `target` number. Let these two numbers be `numbers[index1]` and `numbers[index2]` where `1 <= index1 < index2 <= numbers.length`.

Return the indices of the two numbers, `index1` and `index2`, added by one as an integer array `[index1, index2]` of length 2.

The tests are generated such that **there is exactly one solution**. You may not use the same element twice.

Your solution **must use only constant extra space**.

---

## Examples

### Example 1:
**Input:**  
```plaintext
numbers = [2,7,11,15], target = 9
```
**Output:**  
```plaintext
[1,2]
```
**Explanation:**  
The sum of `2` and `7` is `9`. Therefore, `index1 = 1`, `index2 = 2`. We return `[1, 2]`.

---

### Example 2:
**Input:**  
```plaintext
numbers = [2,3,4], target = 6
```
**Output:**  
```plaintext
[1,3]
```
**Explanation:**  
The sum of `2` and `4` is `6`. Therefore, `index1 = 1`, `index2 = 3`. We return `[1, 3]`.

---

### Example 3:
**Input:**  
```plaintext
numbers = [-1,0], target = -1
```
**Output:**  
```plaintext
[1,2]
```
**Explanation:**  
The sum of `-1` and `0` is `-1`. Therefore, `index1 = 1`, `index2 = 2`. We return `[1, 2]`.

---

## Constraints
- `2 <= numbers.length <= 3 * 10^4`
- `-1000 <= numbers[i] <= 1000`
- `numbers` is sorted in **non-decreasing** order.
- `-1000 <= target <= 1000`
- The tests are generated such that **there is exactly one solution**.


### Answer

This solution doesn't use a hashmap like the previous two-sum question where the array wasn't sorted. Instead, we use the **left** and **right pointers**. The **right pointer** points to the last index, or the largest number in our list, and the **left pointer** points to the first index, or the smallest value. We can then sum the values at the left and right pointers.

Next, if the sum is greater than the target, it means we need a smaller value to reach the target. Since the **right pointer** is at the larger value, we move it one step left to point to the second largest value.

If the sum is less than the target, it means we need a larger value to reach the target. Since the **left pointer** is at the smaller value, we move it one step right to point to the second smallest value.

We keep doing this until the sum equals the target, and we find the solution.

If the **left pointer** and **right pointer** meet or pass each other, it means we didn't find a solution.

#### [Rust Code](./rust/src/lib.rs) 📄💻
#### [Assembly Code](./assembly/two_sum_sorted.asm)