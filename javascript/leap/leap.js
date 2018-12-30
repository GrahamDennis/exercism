export const isLeap = (year) => {
    const isDivisibleBy = (divisor) => year % divisor === 0;
    return isDivisibleBy(4) && (!isDivisibleBy(100) || isDivisibleBy(400));
};
