#ifndef EX03_FILE_H
#define EX03_FILE_H

#include <stddef.h>
#include <stdint.h>

/*
 * Read entire file into buf (at most cap bytes).
 * On success: write bytes read to *out_len and return 0.
 * On error (missing file, too large, I/O): return -1.
 */
int read_file(const char *path, uint8_t *buf, size_t cap, size_t *out_len);

#endif
