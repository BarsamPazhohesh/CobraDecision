function twoSum(nums: number[], target: number): number[] {
  let result: number[] = [];
  for (let i = 0; i < nums.length - 1; i++) {
    for (let j = i + 1; j < nums.length; j++) {
      if (Is_i_j_eq_target({ i: nums[i], j: nums[j], target: target })) {
      }
    }
  }
}
