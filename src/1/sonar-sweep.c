#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

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

    int increases = 0;
    int previous = INT_MAX;

    while ((read = getline(&line, &len, fp)) != -1)
    {
        int num = atoi(line);
        if (num > previous)
        {
            increases++;
        }
        previous = num;
    }

    fclose(fp);
    free(line);

    printf("Total increases: %d\n", increases);

    exit(EXIT_SUCCESS);
}