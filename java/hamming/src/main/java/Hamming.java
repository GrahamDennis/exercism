import java.util.stream.IntStream;

class Hamming {

    private final char[] left;
    private final char[] right;

    Hamming(String left, String right) {
        checkEqualLengths(left, right);
        this.left = left.toCharArray();
        this.right = right.toCharArray();
    }

    private static void checkEqualLengths(String left, String right) {
        if (left.length() != right.length()) {
            throw new IllegalArgumentException("leftStrand and rightStrand must be of equal length.");
        }
    }

    int getHammingDistance() {
        return (int) IntStream.range(0, left.length)
                .filter(this::isDifferentAt)
                .count();
    }

    private boolean isDifferentAt(int idx) {
        return left[idx] != right[idx];
    }
}
