# [Question Link](https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/)

### Problem Description:
You are given an integer array `nums` sorted in non-decreasing order. The goal is to remove duplicates from the array *in-place* such that each unique element appears only once. The relative order of the unique elements should remain the same. Your solution should return the number of unique elements in `nums`.

---

### Requirements:
- Consider the number of unique elements in `nums` to be `k`.
- Modify the array `nums` such that:
  - The first `k` elements contain the unique elements in the same order as they originally appear.
  - The remaining elements can be ignored.
- Return `k`, the total number of unique elements.

---

### Examples:

**Example 1:**  
Input:  
`nums = [1,1,2]`  

Output:  
- `k = 2`  
- Modified array: `[1,2,_]`  

Explanation:  
The function returns `k = 2`, meaning there are 2 unique elements. The first two elements of the array are `[1, 2]`. The underscores (`_`) denote elements that can be ignored.

---

**Example 2:**  
Input:  
`nums = [0,0,1,1,1,2,2,3,3,4]`  

Output:  
- `k = 5`  
- Modified array: `[0,1,2,3,4,_,_,_,_,_]`  

Explanation:  
The function returns `k = 5`, meaning there are 5 unique elements. The first five elements of the array are `[0, 1, 2, 3, 4]`. The underscores (`_`) denote the remaining elements that can be ignored.

---

### Constraints:
- \( 1 \leq nums.length \leq 3 \times 10^4 \)
- \( -100 \leq nums[i] \leq 100 \)
- The array `nums` is sorted in non-decreasing order.



