import java.util.HashMap;
import java.util.Map;
import java.util.function.BiConsumer;

public final class Scrabble {

    private static final Map<Integer, Integer> scoresByCodePoint = createCharacterScores();

    private final int score;

    public static Map<Integer, Integer> createCharacterScores() {
        Map<Integer, Integer> result = new HashMap<>();
        BiConsumer<String, Integer> addScores =
                (letters, score) -> letters.codePoints().forEach(codePoint -> result.put(codePoint, score));

        addScores.accept("AEIOULNRST", 1);
        addScores.accept("DG", 2);
        addScores.accept("BCMP", 3);
        addScores.accept("FHVWY", 4);
        addScores.accept("K", 5);
        addScores.accept("JX", 8);
        addScores.accept("QZ", 10);

        return result;
    }

    public static int getScore(String word) {
        return word.toUpperCase().codePoints().map(Scrabble::getScoreForCodePoint).sum();
    }

    public static int getScoreForCodePoint(int codePoint) {
        return scoresByCodePoint.getOrDefault(codePoint, 0);
    }

    public Scrabble(String word) {
        this.score = getScore(word);
    }

    public int getScore() {
        return score;
    }
}
