# Day 5: Hydrothermal Venture

## Challenge Summary

This challenge involved reading a list of pairs of points. These points represented the beginning and end of a line, and the challenge was to find points on a board which were covered by more than one of these lines.

## Strategy / Lessons Learned

My strategy for this challenge was to define "segments" that consisted of the start point and the end point. The segment would calculate all of the points contained within it. The board was built to be a 2D array with values of zero, and each segment would iterate its points over the board, incrementing the value at that point. A summing function looked through the board for the places where there was at least one overlap of segments.

## Language Used / Comments

For this challenge I tried a language that I had never used before: Ruby. I figured that it would be straightforward and similar to other OOP languages I had used in the past. Unfortunately, it took me a long time to set up Ruby and its linter (?) Rubocop and get it working within my text editor. Even when that was set up, there was an error that showed on every save, which apparently is a [GitHub issue](https://github.com/misogi/vscode-ruby-rubocop/issues/77) that's been open for 3 years. Every save of the file took a couple of seconds as well.

I attempted to implement the solution to the problem, but I wasn't familiar enough with the language to feel productive or to build meaningful abstractions. I was used to statically typed languages, where I could hover over a variable to see what it was and to get intellisense/autocompletion in the editor, and most importantly, for my editor to tell me if something was wrong before I attempted to run it. Having to run the code to find out what was wrong caused context switching I wasn't used to, and not being sure what any data was contained in a variable made things slower. I have more I could say, but the important thing is while my implementation produced the correct result for the sample input from the problem description, it did not produce the correct result for the real input, and I did not want to spend any more time on it, so I decided to call it a day for this problem after a while.
