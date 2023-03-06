#include <stdio.h>
#include "rustffi.h"

int32_t my_callback(int32_t size) {
    char* string = malloc(sizeof(char) * size);
    ffi_get_string(NULL, string);
    printf("%s null\n", string);
    free(string);
}

int main(void) {
    struct FFIDataHandle *h = ffi_lib_init(my_callback);
    poller(h);
    // while(true) {

    // }
}
