#include "ex03_file.h"
#include <stdio.h>

int read_file(const char *path, uint8_t *buf, size_t cap, size_t *out_len) {
  /* TODO: fopen, fread or fgetc loop, fclose — check errors */
  FILE *fp = fopen(path, "rb");
  return -1;
}
