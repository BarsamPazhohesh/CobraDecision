// Q3:
const arraySum = function (array) {
  const target = 9;
  let sumEl;
  let found = false;
  for (let i = 0; i < array.length - 1; i++) {
    // sumEl += array[i];
    sumEl = array[i] + array[i + 1];
    if (sumEl === target) {
      console.log(`You got it: ${array[i]} + ${array[i + 1]} = ${target}`);
      console.log(`Positions: [${i}, ${i + 1}]`);
      found = true;
      break;
    }
  }
  if (!found) {
    console.log(`target not found`);
  }
};
arraySum([2, 7, 11, 15]);
