/* Auto generated file by cbindgen */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Finds primes until a specified number and put them in output buffer. Returns count of numbers
 * it found.
 *
 * Returns -1 if output buffer doesn't have enough capacity
 *
 * # Safety
 *
 * `output_buf` should be a unique mutable pointer and has at least `output_size` capacity.
 */
int32_t primes(int32_t until, int32_t *output_buf, int32_t output_size);
