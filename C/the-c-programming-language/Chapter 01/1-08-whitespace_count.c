#include <stdio.h>

int main()
{
    int c, blanks, tabs, newLines;
    while ((c = getchar()) != EOF) {
        if (c == ' ')
            ++blanks;
        if (c == '\t')
            ++tabs;
        if (c == '\n')
            ++newLines;
    }
    printf("Blanks: %d, Tabs: %d, New Lines: %d\n", blanks, tabs, newLines);
    return 0;
}