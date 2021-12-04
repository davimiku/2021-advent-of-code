#include <vector>
#include <string>
#include <iostream>
#include <fstream>
#include <sstream>
#include <bitset>

const int NUM_DIGITS = 12;

std::vector<std::string> processDigit(int digitIndex, std::vector<std::string> data, bool oxygenGenerator)
{
    std::vector<std::string> zeros;
    std::vector<std::string> ones;

    size_t size = data.size();

    for (int dataIndex = 0; dataIndex < size; dataIndex++)
    {
        auto item = data[dataIndex];
        auto ch = item[digitIndex];
        if (ch == '0')
        {
            zeros.push_back(item);
        }
        else
        {
            ones.push_back(item);
        }
    }
    if (oxygenGenerator)
    {
        if (zeros.size() > ones.size())
        {
            return zeros;
        }
        else
        {
            return ones;
        }
    }
    else
    {
        if (ones.size() < zeros.size())
        {
            return ones;
        }
        else
        {
            return zeros;
        }
    }
}

std::string process(std::vector<std::string> data, bool oxygen)
{
    for (int digitIndex = 0; digitIndex < NUM_DIGITS; digitIndex++)
    {
        auto remainingData = processDigit(digitIndex, data, oxygen);
        data = remainingData;
    }
    return data[0];
}

std::vector<std::string> read_data()
{
    std::vector<std::string> data;

    std::string line;
    std::ifstream dataFile("src/3/data.txt");

    while (std::getline(dataFile, line))
    {
        data.push_back(line);
    }
    return data;
}

int main()
{
    auto data = read_data();

    auto oxygen_str = process(data, true);
    auto co2_str = process(data, false);

    std::bitset<NUM_DIGITS> oxygen(oxygen_str);
    std::bitset<NUM_DIGITS> co2(co2_str);

    ulong oxygen_long = oxygen.to_ulong();
    ulong co2_long = co2.to_ulong();
    ulong result = oxygen_long * co2_long;

    std::cout << result << std::endl;

    return 0;
}
