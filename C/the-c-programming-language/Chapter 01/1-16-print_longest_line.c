#include <stdio.h>

#define MAXLINE 1000

int getLine(char line[], int limit);
void copy(char to[], char from[]);

int main()
{
    char line[MAXLINE];
    char longest[MAXLINE];
    int currentSize = 0;
    int longestSize = 0;

    while ((currentSize = getLine(line, MAXLINE)) > 0)
        if (currentSize > longestSize)
        {
            longestSize = currentSize;
            copy(longest, line);
        }

    if (longestSize > 0)
    {
        printf("%s\n", longest);
    }

    return 0;
}

int getLine(char line[], int limit)
{
    int c;
    int i = 0;
    while ((c = getchar()) != EOF && c != '\n') {
        if (i < limit - 1)
            line[i++] = c;
    }
    if (c == '\n')
        line[i++] = c;
    line[i] = '\0';
    return i;
}

void copy(char to[], char from[])
{
    for (int i = 0; (to[i] = from[i]) != '\0'; i++)
        ;
}