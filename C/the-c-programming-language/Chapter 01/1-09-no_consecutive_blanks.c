#include <stdio.h>

int main()
{
    int c, lastC;
    while ((c = getchar()) != EOF) {
        if (c != ' ' || lastC != ' ')
            putchar(c);
        lastC = c;
    }
    return 0;
}