#include <stdio.h>

#define MAXSTRING 1000

int main()
{
    int c;
    int i = 0;
    char s[MAXSTRING];
    while ((c = getchar()) != EOF)
    {
        if (c == '\t')
        {
            s[i++] = '\\';
            s[i++] = 't';
        }
        else if (c == '\b')
        {
            s[i++] = '\\';
            s[i++] = 'b';
        }
        else if (c == '\\')
        {
            s[i++] = '\\';
            s[i++] = '\\';
        }
        else
            s[i++] = c;
    }
    s[i] = '\0'; 
    printf("%s\n", s);
    return 0;
}
