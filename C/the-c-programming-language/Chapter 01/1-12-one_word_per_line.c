#include <stdio.h>

#define IN 1
#define OUT 0
#define MAXSTRING 1000

int main()
{
    int c, state;
    char s[MAXSTRING];
    int i = 0;
    while ((c = getchar()) != EOF)
    {
        if (c == ' ' || c == '\t' || c == '\n')
        {
            if (state == IN)
            {
                s[i++] = '\n';
                state = OUT;
            }
        }
        else
        {
            state = IN;
            s[i++] = c;
        }
    }
    s[i] = '\0';
    printf("%s", s);
    return 0;
}