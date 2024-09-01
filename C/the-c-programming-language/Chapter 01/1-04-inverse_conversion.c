#include <stdio.h>

int main(void) {
    printf("Celsius to Fahrenheit conversion table");
    for (int cel = 0; cel <= 300; cel = cel + 20)
        printf("Cel: %3d Fahr: %6.1f\n", cel, (9.0/5.0) * cel + 32);
    return 0;
}