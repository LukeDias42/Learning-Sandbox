#include <stdio.h>

#define CHARSAMOUNT 256

int main()
{
    int c;
    int characters[CHARSAMOUNT];
    for (int i = 0; i < CHARSAMOUNT; i++)
        characters[i] = 0;

    while ((c = getchar()) != EOF)
        ++characters[c];

    for (int i = 0; i < CHARSAMOUNT; i++)
        if (characters[i] > 0)
        {
            if (i == '\n')
                printf("\\n: ");
            else if (i == '\t')
                printf("\\t: ");
            else if (i == '\b')
                printf("\\b: ");
            else
                printf("%c: ", i);

            for (int j = 0; j < characters[i]; j++)
                putchar('*');

            putchar('\n');
        }

    return 0;
}