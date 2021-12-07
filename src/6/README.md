# Day 6: Lanternfish

## Challenge Summary

This challenge involved modeling an exponentially growing population of lanternfish. Each fish has a timer that decreases each "day", when the
timer hits 0 the fish resets its timer and a new fish is generated.

## Strategy / Lessons Learned

For the first part, I applied classic OOP principles to "model the real world". I set up a class for `Lanternfish` and encapsulated all of its behavior, and then created a class for `LanternfishSchool`. The school holds many fish, and all of the private behavior of the school was encapsulated in the class and only exposed to `Main` through public methods. It was so **_clean_**.

My simulation ran flawlessly for the first part, iterating through all the fish and growing the school as needed, and I arrived at the correct answer. The second part of the challenge did not change any of the requirements, it only increased the number of "days". However, although the number of days only increased by about 3x, this actually meant the length of the simulation increased by about 4,000,000x. To put this in perspective, if the first part took 500ms to complete, the second part would take more than 23 days. I did not think about any of this at the time. I was so proud of my modeling of the real world, I just incresed the `for` loop from `80` to `256` and clicked "run".

It failed **spectacularly**, crashing with an out-of-memory heap error and completely freezing my entire computer to the point that I had to hard reset it. After letting my computer cool off for the day, I threw away all of my classes and just used a map. The map needed only 8 entries, and each "day" swapped the entries of the map and added more where needed. By using the correct data structure, the answer to the second challenge was calculated very quickly and correctly.

## Language Used / Comments

I used Java for this challenge, as it was familiar to me and I needed to make up for failing yesterday's challenge with Ruby. I believe that the way Java forces you to use a `class` as a fundamental unit makes you tend towards creating classes to "model the real world", such as a Fish class and FishSchool class. However, the challenge was not to model the real world. The challenge was to process data, and the better way of doing that was to treat data as data rather than treating it as objects. If I had employed data-oriented programming and thought about the data before the behavior, I could have arrived at the correct answer with less trouble. Similar to my Python example on Day 2, my carefully crafted `class` relationships turned out to be brittle and had to be removed when the requirements changed.
