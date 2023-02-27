#include <stdio.h>
#include "rustffi.h"

int main(void) {
    int* x;
    int res = my_function(x);
    my_function(NULL);
}