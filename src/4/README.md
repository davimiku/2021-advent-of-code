# Day 4: Giant Squid

## Challenge Summary

This challenge involved playing Bingo with multiple Bingo boards. This challenge was fairly heavy on maintaining the state of the board and mutating it (marking when spaces on the board were reached). The first challenge was to find the first board that achieved a "Bingo" and calculate a result. The second challenge was to find the _last_ board that achieve a "Bingo" and calculate a result.

## Strategy / Lessons Learned

This challenge was on a weekend, so I allowed myself a little more time to complete it than usual. I was able to spend more time planning how to solve the problem and setting up abstractions and utility functions. I implemented the solution in this order:

1. Define the data types
2. Define the functions to implement and their signatures
3. Implement the functions

This took more time up front, but I was able to compose data and functions to build up to the answer. When the second challenge was revealed, in contrast to day 2, I didn't need to change anything about my data structures - only the logic at the top level of the program.

## Language Used / Comments

I used Rust for this challenge. I'm still not yet fully comfortable with Rust, but this wasn't my first time using it either. Rust makes it harder to do in-place mutation and unchecked references, which makes these types of "puzzle" problems more challenging. At the same time, the compiler and type system make it easier to stub out functions while still having the compiler type-check that the result will look as expected. The iterators (where possible) helped to avoid doing manual bookkeeping which often leads to off-by-one errors. Overall, I did feel that I was "fighting" the compiler at first, but when it did finally compile and I ran the program, it produced the correct result on the first try. In this way, it felt frustrating, but it was shifting the troubleshooting from runtime to compiletime. With more experience with Rust, I probably would be able to complete that troubleshooting faster.
