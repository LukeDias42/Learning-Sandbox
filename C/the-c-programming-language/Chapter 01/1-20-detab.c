#include <stdio.h>

#define DEFAULT_TAB_SIZE 8
#define MAX_LINE 1024

int calculateSpaces(int tabSize, int offset);
int getLine(char line[], int limit);
int detab(char line[], int tabSize);

int main()
{
    int tabSize = DEFAULT_TAB_SIZE;
    int size;
    char line[MAX_LINE];

    while ((size = getLine(line, MAX_LINE)) > 0) {
        size = detab(line, tabSize);
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

int calculateSpaces(int tabSize, int offset)
{
    return tabSize - (offset % tabSize);
}

int detab(char line[], int tabSize)
{
    int offset = 0;
    for ( int i = 0; line[i] != '\0'; i++) {
        if (line[i] == '\t') {
            int spaces = calculateSpaces(tabSize, offset);
            for (int i = 0; i < spaces; i++) {
                putchar(' ');
                offset++;
            }
        }
        else {
            putchar(line[i]);
            offset++;
        }
    }

    return offset;
}
