# Jump Game

You are given an integer array `nums`. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

Return `true` if you can reach the last index, or `false` otherwise.

### Example 1:
**Input:** `nums = [2,3,1,1,4]`  
**Output:** `true`  
**Explanation:** Jump 1 step from index 0 to 1, then 3 steps to the last index.

### Example 2:
**Input:** `nums = [3,2,1,0,4]`  
**Output:** `false`  
**Explanation:** You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.

### Constraints:
- `1 <= nums.length <= 10^4`
- `0 <= nums[i] <= 10^5`



## Answer

We can solve this problem using a **greedy approach** instead of trying all paths, which would take too long ⏳. But I'm not certain whether a greedy approach will work for all cases 🤔.

The main idea is:

- We keep track of the **maximum index** we have reached so far (`max_reach`) 📍.
- We iterate through the array of numbers 🔢, updating `max_reach` in each step 🔄.
- We check whether the present index is greater than `max_reach` 📉, i.e., we are stuck 🚫 and cannot move further, so we return `false` ❌.

  **For Example:** Let us assume we have the input `nums = [1, 2, 0, 1, 0, 1]`, at index `4`, we are stuck because `max_reach` is not enough to move further 🚧.

- Also, in the appropriate context, we check whether `max_reach` reaches or passes the ending index 🎯, under such circumstance returning `true` ✅.

You can check the Rust code here:  
#### [Rust Code](./rust/src/lib.rs) 📄💻
#### [Assembly Code](./assembly/jump_code.asm)
