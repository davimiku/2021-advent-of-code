#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

const int WINDOW_SIZE = 3;
const int NUM_WINDOWS = 4;

/**
 * "data.txt" contains a newline separated list of integers
 * from the problem data.
 */

int main()
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    fp = fopen("data.txt", "r");
    if (fp == NULL)
    {
        exit(EXIT_FAILURE);
    }

    int window_sums[4] = {0, 0, 0, 0};
    int window_counts[4] = {0, -1, -2, -3};

    int increases = 0;
    int previous = INT_MAX;

    while ((read = getline(&line, &len, fp)) != -1)
    {
        int num = atoi(line);

        for (int i = 0; i < NUM_WINDOWS; i++)
        {
            if (window_counts[i] >= 0 && window_counts[i] < WINDOW_SIZE)
            {
                window_sums[i] += num;
                window_counts[i]++;
            }
            else if (window_counts[i] < 0)
            {
                window_counts[i]++;
            }

            if (window_counts[i] == WINDOW_SIZE)
            {
                int current_sum = window_sums[i];
                if (current_sum > previous)
                {
                    increases++;
                }
                previous = current_sum;
                window_sums[i] = 0;
                window_counts[i] = -1; // let this slot take a break
            }
        }
    }

    fclose(fp);
    free(line);

    printf("Total increases: %d\n", increases);

    exit(EXIT_SUCCESS);
}