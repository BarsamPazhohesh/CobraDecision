# Two Sum 🧮

Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

## Examples 📝

### Example 1:
**Input:**  
`nums = [2,7,11,15]`, `target = 9`  
**Output:**  
`[0,1]`  
**Explanation:** Because `nums[0] + nums[1] == 9`, we return `[0, 1]`.

### Example 2:
**Input:**  
`nums = [3,2,4]`, `target = 6`  
**Output:**  
`[1,2]`

### Example 3:
**Input:**  
`nums = [3,3]`, `target = 6`  
**Output:**  
`[0,1]`

## Constraints ⚖️

- `2 <= nums.length <= 10^4`
- `-10^9 <= nums[i] <= 10^9`
- `-10^9 <= target <= 10^9`

---

## Answer 💡
The most straightforward and first solution to the "Two Sum" problem is a brute-force solution, where we loop through the array a couple of times to see if two numbers sum up to the target. This solution, though, has a time complexity of **O(n²)**, which is not ideal.

For this, we can think outside the box and use a **hash map**. 🔑

Let's consider an example: for the array `[3, 1, 4, 5]` and target sum 6, in the brute-force solution, we would have two loops nested inside each other—one to pick a number and another to search among the rest of the numbers in the array. For every picked number, we would look for the target minus the picked number somewhere else in the array.  
But this can be significantly optimized using a hash map. The concept is to put the difference (`target - selected_value`) as the key and the index of the number in the array as the value in the hash map. This allows us to decrease the time complexity from **O(n²)** to **O(n)**. 🚀

## How the Algorithm Works 🔍

1. **Iterate through the array**: Begin by iterating through the array and for each number, find the difference (`diff = target - current_number`).

2. **Check if the difference is in the hash map**: If the difference is in the hash map, then we have found the pair whose sum is the target. We can then return the index of the current number and the index of the number we found in the hash map.  
   If we can't find a pair, **put the number in the map**: If the difference is not in the map, we put the current number and its index in the hash map for later lookups. 🗂️

3. **Complete the iteration**: If no pair is found by loop termination, we return `null` or signal that there is no solution. ❌

## Why a Hash Map Works 🧠

The important observation here is that rather than looking for the complement (`target - chosen number`) in the remaining numbers (which would be **O(n)** time for each number), we insert the complement into a hash map as we proceed. This allows subsequent lookups to be constant time (**O(1)**). 🔄

## Time Complexity ⏱️

- **Time Complexity**: **O(n)** — The algorithm has to go through the array only once, and lookup and insertion in the hash map take constant time.  
- **Space Complexity**: **O(n)** — Worst-case scenario, the hash map will have all the numbers of the array.

This optimization reduces the time complexity from **O(n²)** in the brute-force solution to **O(n)**, making it much more efficient. 💨
#### [Zig Code](./src/main.zig) 📄💻