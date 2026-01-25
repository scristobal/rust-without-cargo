#include <stdatomic.h>

_Atomic int global_counter = 0;
__thread int local_counter = 0;

int increment(void) {
    atomic_fetch_add(&global_counter, 1);
    local_counter++;
    return local_counter;
}
