# Day 1: Sonar Sweep

## Challenge Summary

This challenge entailed reading a sequence of numbers and determining
how many numbers were larger than the previous number. The second part
of the challenge was the same thing, except tracking how many "windows"
of size 3 had a larger sum than the previous "window".

## Strategy / Lessons Learned

With the first challenge, we only needed to keep a variable
of the previous number to compare and increment a counter. The
second challenge changed the strategy to need to keep track of
four "windows" as well as the sum of the previously completed window.

When each window "completed", the sum of that window was calculated
and compared to the previous sum.

At first, I had an "off-by-one" error because I was not checking the
sum of the final completed window. This was solved by re-arranging some
`if` conditions to ensure the completed windows were always counted.

## Language Used / Comments

I used C for this challenge. The hardest part was setting up
reading in the file and remembering how to convert from `char*` to `int`,
but after that, the logic to actually solve the puzzle was
very straightforward imperative logic with the C-style 3-pronged
`for` loop to perform in-place mutation.
