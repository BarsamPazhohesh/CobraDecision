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
  }
  return [];
}
