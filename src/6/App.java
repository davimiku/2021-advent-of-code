import java.util.HashMap;

public class App {
    static void partOne() {
        var timers = FileParser.parse("data.txt");

        var school = new LanternfishSchool(timers);

        for (int day = 0; day < 80; day++) {
            school.processFishes();
            System.out.println("After day " + day + ", num fishes: " + school.size());
        }

        System.out.println("Num fishes: " + school.size());
    }

    static void partTwo() {
        var days = FileParser.parse("data.txt");

        var map = new HashMap<Integer, Long>();

        for (int i = 0; i <= 8; i++) {
            map.put(i, (long) 0);
        }
        for (int day : days) {
            map.put(day, map.get(day) + 1);
        }

        System.out.println(map);

        for (int day = 0; day < 256; day++) {

            long next = map.get(8);
            for (int age = 7; age > 0; age--) {
                long temp = map.get(age);
                map.put(age, next);
                next = temp;
            }
            long zeros = map.get(0);
            map.put(0, next);
            map.put(8, zeros);
            map.put(6, map.get(6) + zeros);
        }

        long sum = map.values().stream().reduce((long) 0, Long::sum);

        System.out.println(map);
        System.out.println("Sum: " + sum);
    }

    public static void main(String[] args) {
        partTwo();
    }
}
