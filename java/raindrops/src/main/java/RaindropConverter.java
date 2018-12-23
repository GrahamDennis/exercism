class RaindropConverter {

    boolean isDivisibleBy(int number, int divisor) {
        return (number % divisor) == 0;
    }

    String convert(int number) {
        StringBuilder stringBuilder = new StringBuilder();
        if (isDivisibleBy(number, 3)) {
            stringBuilder.append("Pling");
        }
        if (isDivisibleBy(number, 5)) {
            stringBuilder.append("Plang");
        }
        if (isDivisibleBy(number, 7)) {
            stringBuilder.append("Plong");
        }
        String result = stringBuilder.toString();
        if (!result.isEmpty()) {
            return result;
        } else {
            return Integer.toString(number);
        }
    }

}
