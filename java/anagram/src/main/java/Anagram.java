import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class Anagram {
    private final String word;

    public Anagram(String word) {
        this.word = word;
    }

    public List<String> match(List<String> candidates) {
        Map<Integer, Integer> targetCharacterCounts = characterCounts(word);
        return candidates.stream()
                .filter(candidate -> !candidate.equalsIgnoreCase(word))
                .filter(candidate -> characterCounts(candidate).equals(targetCharacterCounts))
                .collect(Collectors.toList());
    }

    private Map<Integer, Integer> characterCounts(String word) {
        return word.chars()
                .map(Character::toLowerCase)
                .boxed()
                .collect(Collectors.toMap(c -> c, c -> 1, (left, right) -> left + right));
    }
}
