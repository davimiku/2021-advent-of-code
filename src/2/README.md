# Day 2: Dive!

## Challenge Summary

Today's challenge was quite straightforward, essentially reading input
and conditionally modifying variables based on that input.

## Strategy / Lessons Learned

For the first part of the exercise, I experimented with a class hierarchy
and overloading the `+` operator to be able to directly add the `Depth` and
`Horizontal` values as classes.

The second part of the exercise exposed cross-cutting concerns between the
classes I had set up, and the hierarchy no longer applied. It was easier to
completely delete the hierarchy and operate on the data directly than to try
to adjust the hierarchy to the changed requirements. I think this was a bit of
a lesson by itself, even for a small example.

## Language Used / Comments

I used Python for this challenge, which I regret in the sense that today's challenge
was fairly easy and I wish I had saved Python for a harder challenge. Python is a
nice language for challenges such as this, it makes it pretty easy to get up and
running towards the solution without much overhead.
