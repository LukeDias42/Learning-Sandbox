#include <stdio.h>
#define MAXLINE 1000

int getLine(char line[], int limit);

int main()
{
    int size = 0;
    char string[MAXLINE];

    while((size = getLine(string, MAXLINE)) > 0)
        if (size > 80)
            printf("%s", string);
    
    return 0;
}

int getLine(char line[], int limit)
{
    int c;
    int i = 0;
    while (i < limit - 1 && (c = getchar()) != EOF && c != '\n') {
        line[i++] = c;
    }
    
    if (c == '\n')
        line[i++] = '\n';
    
    line[i] = '\0';
    return i;
}