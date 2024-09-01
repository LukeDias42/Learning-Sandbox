#include <stdio.h>

float fahrToCel(int fahr);

int main() 
{
    for (int fahr = 0; fahr <= 300; fahr += 20)
        printf("Fahr: %3d  Cel: %6.2f\n", fahr, fahrToCel(fahr));
    return 0;
}

float fahrToCel(int fahr)
{
    return (5.0/9.0) * (fahr - 32);
}