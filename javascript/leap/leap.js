export const isLeap = (year) => {
    // if year is divisible by 400, then it's a leap year
    if ((year % 400) === 0) return true;
    // otherwise if it's divisible by 100, then it isn't
    if ((year % 100) === 0) return false;
    // otherwise if it's divisible by 4, then it is
    if ((year % 4) === 0) return true;
    // otherwise it's not a leap year
    return false;
}