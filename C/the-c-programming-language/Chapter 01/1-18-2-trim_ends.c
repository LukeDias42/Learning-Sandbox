#include <stdio.h>

#define MAXSIZE 1000
#define IN 1
#define OUT 0
#define NOTFOUND -1

int getLine(char line[], int limit);
int trimEnds(char line[], int size);
int isWhitespace(char c);

int main()
{
    char line[MAXSIZE];
    int size;
    while ((size = getLine(line, MAXSIZE)) > 0) {
        size = trimEnds(line, size);
        if (size != 0){
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
    return i - 1;
}

int trimEnds(char line[], int size)
{
    int i;
    int start = NOTFOUND, end = NOTFOUND;
    for (i = 0; i < size; i++) {
        if (start == -1 && !isWhitespace(line[i])) {
            start = i;
        }
        if (end == -1 && !isWhitespace(line[size - 1 - i])) {
            end = size - 1 - i;
        }
        if (start > -1 && end > -1) {
            break;
        }
    }

    if (start == NOTFOUND || end == NOTFOUND){
        line[0] = '\0';
        return 0;
    }

    if (start == 0 && end == size -1) return size;

    for (i = 0; i < end - start + 1; i++) {
        line[i] = line[start + i];
    }

    line[i] = '\n';
    line[i+1] = '\0';
    return i;
}

int isWhitespace(char c)
{
    return c == ' ' || c == '\t';
}

