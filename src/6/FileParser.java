import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class FileParser {
    static public int[] parse(String path) {
        String contents = "";
        try {
            contents = Files.readString(Path.of(path));
        } catch (IOException e) {
            System.err.println("Whoopsy!");
        }

        var split = contents.split(",");
        int[] ints = new int[split.length];

        for (int i = 0; i < split.length; i++) {
            ints[i] = Integer.parseInt(split[i]);
        }

        return ints;
    }
}
