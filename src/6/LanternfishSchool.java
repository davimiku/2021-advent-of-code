import java.util.ArrayList;

public class LanternfishSchool {
    public ArrayList<Lanternfish> fishes;

    public LanternfishSchool(int[] timers) {
        var fishes = new ArrayList<Lanternfish>();

        for (int timer : timers) {
            fishes.add(new Lanternfish(timer));
        }

        this.fishes = fishes;
    }

    public void processFishes() {
        var newFishes = new ArrayList<Lanternfish>();
        for (var fish : fishes) {
            fish.decrementTimer();
            if (fish.justSpawnedNewFish) {
                newFishes.add(new Lanternfish(8));
            }
        }
        fishes.addAll(newFishes);
    }

    public int size() {
        return fishes.size();
    }
}
