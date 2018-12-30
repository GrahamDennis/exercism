export const isLeap = (year) => {
    const isDivisibleBy = (divisor) => year % divisor === 0;
    return isDivisibleBy(400) || isDivisibleBy(4) && !isDivisibleBy(100);
};
