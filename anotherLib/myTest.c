#include <stdio.h>

void FFIPrintTest()
{
    printf("FFPrintTest\n");
}

int FFIScanTest()
{
    printf("FFIScanTest\n");
    int a;
    scanf("%d", &a);
    return a;
}

void FFIPrintTestWithArg(char *some,int len)
{
    printf("FFIPrintTestWithArg %s\n", some);
}

char *AnoTest()
{
    char *name = "Nishanth";
    return name;
}