#include <stdio.h>

int getLine(char line[], int limit);
int replaceBlanks(char line[], int tabSize);

int main()
{
    char line[1024];
    int size;
    while ((size = getLine(line, 1024)) > 0) {
        replaceBlanks(line, 8);
    }

    return 0;
}

int replaceBlanks(char line[], int tabSize)
{
    int newSize = 0;
    int blankSequence = 0, offset = 0;
    for (int i = 0; line[i] != '\0'; i++) {
        if (line[i] == ' ') {
            blankSequence++;
            if ((i + 1) % offset == 0) {
                putchar('\t');
                offset += tabSize;
                blankSequence = 0;
                newSize++;
            }
        }
        else {
            for (; blankSequence > 0; blankSequence--) {
                putchar(' ');
                newSize++;
            }
            putchar(line[i]);
            newSize++;
        }
    }
    return newSize;
}

int getLine(char line[], int limit)
{
    int c, i = 0;
    while (i < limit && (c = getchar()) != EOF && c != '\n') {
        line[i++] = c;
    }

    if (c == '\n') {
        line[i++] = c;
    }

    line[i] = '\0';

    return i - 1;
}

