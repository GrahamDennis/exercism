import java.util.stream.Collectors;

class IsogramChecker {

    boolean isIsogram(String phrase) {
        return phrase.chars()
                // Skip hyphens and spaces
                .filter(c -> c != '-' && c != ' ')
                // Duplicate checking is case-insensitive so map all characters to the lowercase form
                .map(Character::toLowerCase)
                .boxed()
                // Create a map from character to the number of occurrences of that character
                .collect(Collectors.toMap(c -> c, c -> 1, (left, right) -> left + right))
                .entrySet().stream()
                // Return true if each character appears exactly once
                .allMatch(entry -> entry.getValue() == 1);
    }

}
