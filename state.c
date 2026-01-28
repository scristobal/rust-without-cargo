/**
 * state.c - Thread-safe state management with atomic and thread-local storage
 *
 * This file demonstrates the use of C11 atomic operations and thread-local
 * storage for managing both shared and per-thread state.
 */

#include <stdatomic.h>

_Atomic int global_counter = 0;
__thread int local_counter = 0;

int increment(void) {
    atomic_fetch_add(&global_counter, 1);
    local_counter++;
    return local_counter;
}
