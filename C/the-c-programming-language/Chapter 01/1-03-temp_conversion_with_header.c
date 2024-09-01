#include <stdio.h>

int main(void) {
    printf("Fahrenheit to Celsius conversion table");
    for (int fahr = 0; fahr <= 300; fahr = fahr + 20)
        printf("Fahr: %3d Cel: %6.1f\n", fahr, (5.0/9.0) * (fahr-32));
    return 0;
}