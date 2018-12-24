class Hamming {

    private final String leftStrand;
    private final String rightStrand;

    Hamming(String leftStrand, String rightStrand) {
        checkEqualLengths(leftStrand, rightStrand);
        this.leftStrand = leftStrand;
        this.rightStrand = rightStrand;
    }

    private static void checkEqualLengths(String leftStrand, String rightStrand) {
        if (leftStrand.length() != rightStrand.length()) {
            throw new IllegalArgumentException("leftStrand and rightStrand must be of equal length.");
        }
    }

    int getHammingDistance() {
        char[] leftCharArray = leftStrand.toCharArray();
        char[] rightCharArray = rightStrand.toCharArray();

        int distance = 0;
        for (int idx = 0; idx < leftCharArray.length; idx++) {
            if (leftCharArray[idx] != rightCharArray[idx]) {
                distance++;
            }
        }

        return distance;
    }

}
