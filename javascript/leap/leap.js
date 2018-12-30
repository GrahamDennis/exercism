const isDivisibleBy = (divisor) => (dividend) => (dividend % divisor) === 0;

export const isLeap = (year) => isDivisibleBy(400)(year) || isDivisibleBy(4)(year) && !isDivisibleBy(100)(year);
