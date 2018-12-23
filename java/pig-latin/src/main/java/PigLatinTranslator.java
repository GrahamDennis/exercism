import java.util.Arrays;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class PigLatinTranslator {
    private static final Pattern startingConsonants = Pattern.compile(
            "^("
                    // x or y followed by a vowel should be treated as a consonant
                    // otherwise x or y followed by consonants will be treated as a single consonant cluster
                    + "[xy](?=[aeiou])"
                    // match a consonant cluster
                    + "|[^aeiouxy]*)" +
                    // If the consonant cluster ends with q, add the following u if present
                    "((?<=q)u)?",
            Pattern.COMMENTS);

    public String translate(String input) {
        return Arrays.stream(input.split(" "))
                .map(this::translateWord)
                .collect(Collectors.joining(" "));
    }

    private String translateWord(String word) {
        StringBuffer buffer = new StringBuffer();
        Matcher matcher = startingConsonants.matcher(word);
        matcher.find();

        matcher.appendReplacement(buffer, "");
        matcher.appendTail(buffer);
        buffer.append(matcher.group(0));
        buffer.append("ay");

        return buffer.toString();
    }
}
