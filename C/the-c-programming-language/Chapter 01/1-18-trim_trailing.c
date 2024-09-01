#include <stdio.h>

#define MAXSIZE 1000

int getLine(char line[], int limit);
int trimTrailing(char line[], int size);

int main()
{
    char line[MAXSIZE];
    int size = 0;
    while ((size = getLine(line, MAXSIZE)) > 0) {
        size = trimTrailing(line, size);
        if (size > 0) {
            printf("%s", line);
        }
    }

    return 0;
}

int getLine(char line[], int limit) 
{
    int c, i = 0;
    while (i < limit && (c = getchar()) != EOF && c != '\n') {
        line[i++] = c;
    }
    
    if (c == '\n') {
        line[i++] = '\n';
    }

    line[i] = '\0';
    return i;
}

int trimTrailing(char line[], int size)
{
    int i;
    for (i = size - 2; line[i] == '\t' || line[i] == ' '; i--)
        ; 
    line[++i] = '\n';
    line[++i] = '\0';
    
    return i;
}

