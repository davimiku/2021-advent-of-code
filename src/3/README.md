# Day 3: Binary Diagnostic

## Challenge Summary

This challenge involved reading a file of data and converting between various formats - string, int, and binary.

## Strategy / Lessons Learned

The first challenge could be done in a streaming fashion, by calculating sums up front. From those sums, it involed converting formats between
strings, integers, and binary. I completed the second challenge by loading all of the data in a vector, because it needed to be iterated over multiple times.
The vector was partitioned into increasingly smaller partitions based on the criteria until only 1 element remained, which were then converted to a bitset and an integer for the final answer.

## Language Used / Comments

I used C++ for this challenge. The hardest part was remembering the imports. Most online references leave out the imports (for brevity, I guess?) which causes confusing error messages. A lot of answers seemingly (and invisibly) do `using namespace std;` as well, making it more difficult to understand the imports.

For example:

```cpp
void main()
{
    // snip
    std::ifstream dataFile;
    // snip
}
```

This shows an error message on `dataFile` (not `std::ifstream`) saying "incomplete type is not allowed". This lead me down a rabbit hole of trying to figure out if I declared my variable incorrectly (because the error was on the variable, not the type). I just needed to `#include <fstream>`, which in hindsight, made me feel silly for forgetting it.
