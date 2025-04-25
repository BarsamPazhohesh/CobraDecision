function FiboCal(limiter: number): number[] {
  if (limiter > 0) {
    let returnNums: number[] = [0];
    if (limiter == 1) {
      return returnNums;
    }

    returnNums.push(1);
    if (limiter == 2) {
      return returnNums;
    }

    for (let i = 1; i < limiter - 1; i++) {
      let temp = returnNums[i] + returnNums[i - 1];

      returnNums.push(temp);
    }
    return returnNums;
  }
  return [];
}

FiboCal(8)?.forEach((e) => {
  console.log(e);
});
