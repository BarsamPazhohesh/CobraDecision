# Remove Duplicates from Sorted Array

Given an integer array `nums` sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in `nums`.

Consider the number of unique elements of `nums` to be `k`, to get accepted, you need to do the following things:

- Change the array `nums` such that the first `k` elements of `nums` contain the unique elements in the order they were present in `nums` initially. The remaining elements of `nums` are not important as well as the size of `nums`.
- Return `k`.

## Custom Judge:

The judge will test your solution with the following code:

```java
int[] nums = [...]; // Input array
int[] expectedNums = [...]; // The expected answer with correct length

int k = removeDuplicates(nums); // Calls your implementation

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
```

If all assertions pass, then your solution will be accepted.

## Example 1:

**Input:** `nums = [1,1,2]`  
**Output:** `2`, `nums = [1,2,_]`  
**Explanation:** Your function should return `k = 2`, with the first two elements of `nums` being 1 and 2 respectively. It does not matter what you leave beyond the returned `k` (hence they are underscores).

## Example 2:

**Input:** `nums = [0,0,1,1,1,2,2,3,3,4]`  
**Output:** `5`, `nums = [0,1,2,3,4,_,_,_,_,_]`  
**Explanation:** Your function should return `k = 5`, with the first five elements of `nums` being 0, 1, 2, 3, and 4 respectively. It does not matter what you leave beyond the returned `k` (hence they are underscores).

## Constraints:

- `1 <= nums.length <= 3 * 10^4`
- `-100 <= nums[i] <= 100`
- `nums` is sorted in non-decreasing order.

## Answer

We can solve this problem with a **two-pointer approach** to erase duplicates from the sorted array in a most efficient way. This will allow us to modify the array in-place without utilizing extra space. ⚡💡

The approach is:

- We have two pointers: one pointer (`index`) will traverse the array, and another pointer (`unique_index`) will keep the place where the next unique element should be kept. 📍🔄

- As we are iterating through the array, if the current element is different from the previous one, we assign it at the `unique_index` and increment it. ➡️

- The array size will be equal to the `unique_index` at the end, which will be the count of unique elements in the array. 🎯

**For Instance:** Given the input `nums = [1, 1, 2, 2, 3]`, the function will return `3`, and the first `3` elements of `nums` will be `[1, 2, 3]`. The remaining elements are not counted. ❌


#### [Rust Code](./src/lib.rs) 📄💻
