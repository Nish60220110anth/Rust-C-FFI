#include <stdio.h>

float AddFloat(float f1, float f2)
{
    float res = f1 + f2;
    printf("Hello from C\n");
    printf("I got  %f and %f => %f\n", f1, f2, res);
    return res;
}