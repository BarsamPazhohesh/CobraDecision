var _a;
function FiboCal(limiter) {
    if (limiter > 0) {
        var returnNums = [0];
        if (limiter == 1) {
            return returnNums;
        }
        returnNums.push(1);
        if (limiter == 2) {
            return returnNums;
        }
        for (var i = 1; i < limiter - 1; i++) {
            var temp = returnNums[i] + returnNums[i - 1];
            returnNums.push(temp);
        }
        return returnNums;
    }
    return [];
}
(_a = FiboCal(8)) === null || _a === void 0 ? void 0 : _a.forEach(function (e) {
    console.log(e);
});
