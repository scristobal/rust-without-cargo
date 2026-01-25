#include <stdio.h>

extern const char* rust_callback(const char* s);

static char buffer[256];

const char* c_function(const char* s) {
    snprintf(buffer, sizeof(buffer), "%s C", s);
    return rust_callback(buffer);
}
