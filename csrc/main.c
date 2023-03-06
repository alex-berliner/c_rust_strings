#include <stdio.h>
#include "rustffi.h"

int32_t my_callback(int32_t a, int32_t b) {
    printf("callback: %d\n", a+b);
}

int main(void) {
    struct FFIDataHandle *h = lib_init(my_callback);
    printf("runned\n");
    poller(h);
    while(true) {

    }
}