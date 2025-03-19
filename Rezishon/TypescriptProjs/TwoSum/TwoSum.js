//#region Main function
/**
 * The main function
 * (Start point)
 */
function main() {
    console.log(twoSum([2, 7, 11, 15], 9));
    console.log(twoSum([3, 2, 4], 6));
    console.log(twoSum([3, 3], 6));
}
main();
//#endregion
//#region Question logic
/**
 * @summary This method will check sum of what two numbers will be the target
 * @param nums Number of arrays we will iterate over it
 * @param target The target of sum of two numbers
 * @returns Array of numbers which are index of true numbers
 */
function twoSum(nums, target) {
    var result = [];
    for (var i = 0; i < nums.length - 1; i++) {
        for (var j = i + 1; j < nums.length; j++) {
            if (Is_i_j_eq_target({ i: nums[i], j: nums[j], target: target })) {
                result.push(i);
                result.push(j);
                return result;
            }
        }
    }
    return [];
}
//#endregion
//#region Repository
/**
 * This method will check the question condition
 * @param inputValues An interface which contains i, j, & target that are all number type
 * @returns boolean
 */
function Is_i_j_eq_target(inputValues) {
    return inputValues.i + inputValues.j === inputValues.target;
}
//#endregion
