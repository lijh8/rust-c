#include <string.h>
#include "hello.h"

char *strncpy2(char *dest, const char *src, size_t n)
{
    strncpy(dest, src, n);
    return dest;
}

int *intcpy(int *dest, const int *src)
{
    *dest = *src;
    return dest;
}
