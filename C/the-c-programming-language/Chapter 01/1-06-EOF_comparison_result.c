#include <stdio.h>

int main()
{
    printf("Character typed compared to EOF: %d\n", getchar() == EOF);
    return 0;
}