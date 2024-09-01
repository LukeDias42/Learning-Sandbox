#include <stdio.h>

#define MAXSIZE 1000

int getLine(char line[], int limit);
void reverseLine(char line[], int size);

int main()
{
    char line[MAXSIZE];
    int size;
    while ((size = getLine(line, MAXSIZE)) > 0) {
        reverseLine(line, size);
        printf("%s", line);
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
        line[i++] = c;
    }

    line[i] = '\0';

    return i - 1;
}

void reverseLine(char line[], int size) 
{
    if (size == 1) return;
    for (int i = 0; i < size>>1; i++) {
        char temp = line[i];
        line[i] = line[size - i - 1];
        line[size - i - 1] = temp;
    }
}
