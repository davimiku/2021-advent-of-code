public class Lanternfish {
    private int timer;
    public boolean justSpawnedNewFish;

    private static int DEFAULT_TIMER = 6;

    public Lanternfish() {
        this(DEFAULT_TIMER);
    }

    public Lanternfish(int timer) {
        this.timer = timer;
        justSpawnedNewFish = false;
    }

    /**
     * This function does two things, decrements a timer with wraparound
     * and returns true if it wrapped around.
     * 
     * @return true if wraparound happened, false if it did not
     */
    public void decrementTimer() {
        justSpawnedNewFish = false;
        timer--;
        if (timer < 0) {
            timer = 6;
            justSpawnedNewFish = true;
        }
    }
}
