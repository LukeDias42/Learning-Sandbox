#include <stdio.h>

#define MAXLENGTH 100
#define IN 1
#define OUT 0

int main()
{
    int c, lengths[MAXLENGTH];
    int wordLength = 0;
    int state = OUT;
    for (int i = 0; i < 10; ++i)
        lengths[i] = 0;
    while ((c = getchar()) != EOF) {
        if (c == ' ' || c == '\t' || c == '\n')
        {
            if (state == IN && wordLength > 0)
            {
                ++lengths[wordLength];
                wordLength = 0;
                state = OUT;
            }
        }
        else if (state == OUT)
        {
            state = IN;
            ++wordLength;
        }
        else
            ++wordLength;
    }

    for (int i = 0; i < MAXLENGTH; ++i)
    {
        if (lengths[i] > 0)
        {
            printf("%d: ", i);
            for (int j = 0; j < lengths[i]; ++j)
                printf("*");
            printf("\n");
        }
    }

    return 0;
}