#include <stdio.h>
#include "rustffi.h"
struct FFIDataHandle *h;
int32_t my_callback(int32_t size) {
    if (NULL == h) {
        return -1;
    }
    char* string = malloc(sizeof(char) * size);
    printf("%s null", (NULL == string)?"is":"not");
    ffi_get_string(h, string);
    printf("%s null", (NULL == string)?"is":"not");
    free(string);
}

int main(void) {
    h = ffi_lib_init(my_callback);
    poller(h);
    // while(true) {

    // }
}
