#include "ex03_file.h"
#include <stdio.h>

int read_file(const char *path, uint8_t *buf, size_t cap, size_t *out_len) {
  /* TODO: fopen, fread or fgetc loop, fclose — check errors */
  FILE *fp = fopen(path, "rb");

  if (fp == NULL) {
    return -1;
  }

  size_t bytes_read = fread(buf, 1, cap, fp);
  return 0;
}
