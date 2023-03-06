#include <stdio.h>
#include "rustffi.h"

int32_t my_callback(int32_t a, int32_t b) {
    // react to event
}

int main(void) {
    struct FFIDataHandle *h = reg_event_callback(my_callback);
    printf("runned\n");
    poller();
    while(true) {

    }
}