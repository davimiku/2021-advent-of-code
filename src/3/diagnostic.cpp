#include <vector>
#include <string>
#include <iostream>
#include <fstream>
#include <sstream>
#include <bitset>
#include <stdlib.h>

const int NUM_DIGITS = 12;

void process_line(int bit_counts[NUM_DIGITS], std::string line)
{
    std::istringstream ss(line);
    std::vector<int> result;

    char c;
    int i = 0;
    while (ss >> c)
    {
        // convert '0' and '1' to 0 and 1
        int bit = c - '0';

        if (bit)
        {
            bit_counts[i]++;
        }
        else
        {
            bit_counts[i]--;
        }
        i++;
    }
}

void update_bit_counts(int bit_counts[NUM_DIGITS])
{
    std::vector<int> ints;

    std::string line;
    std::ifstream dataFile("src/3/data.txt");

    while (std::getline(dataFile, line))
    {
        process_line(bit_counts, line);
    }

    std::string joined = "";
    std::string joined_reverse = "";
    for (int i = 0; i < NUM_DIGITS; i++)
    {
        if (bit_counts[i] > 0)
        {
            joined.push_back('1');
            joined_reverse.push_back('0');
        }
        else
        {
            joined.push_back('0');
            joined_reverse.push_back('1');
        }
    }

    std::bitset<NUM_DIGITS> forward(joined);
    std::bitset<NUM_DIGITS> flipped(joined_reverse);

    ulong forward_long = forward.to_ulong();
    ulong flipped_long = flipped.to_ulong();
    ulong result = forward_long * flipped_long;

    std::cout << forward_long << std::endl;
    std::cout << flipped_long << std::endl;
    std::cout << result << std::endl;
}

int main()
{
    int bit_counts[NUM_DIGITS] = {0};
    update_bit_counts(bit_counts);

    return 0;
}
