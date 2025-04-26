//#region Main function
/**
 * The main function
 * (Start point)
 */
function main() {
    console.log(twoSum([5, 25, 75], 100));
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
    var result = new Map();
    var index = 0;
    var num = nums[index];
    for (var i = 1; i <= nums.length;) {
        console.log("".concat(index, " - ").concat(i));
        if (Is_i_j_eq_target({ i: num, j: nums[i], target: target })) {
            console.log("in");
            result.set(0, index + 1);
            result.set(1, i + 1);
            return Array.from(result.values());
        }
        if (i === nums.length) {
            index++;
            num = nums[index];
            i = index;
        }
        i++;
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
