// Q4 :

const twoSum = function (array, target) {
  let sumOfEl = 0;
  let targetFound = false;
  for (let i = 0; i < array.length; i++) {
    for (let j = i + 1; j < array.length; j++) {
      sumOfEl = array[i] + array[j];
      if (sumOfEl === target) {
        console.log(`your target is ${sumOfEl}`);
        console.log([i + 1, j + 1]);
        targetFound = true;
        return;
      }
    }
  }
  if (targetFound === false) {
    console.log(`target does not found!`);
  }
};
twoSum([2, 7, 11, 15], 9);
