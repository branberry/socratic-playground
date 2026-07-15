#include "ex01_bytes.h"

void reverse_bytes(uint8_t *buf, size_t len) {

  if (len <= 1) {
    return;
  }

  /* TODO: two pointers from each end, swap until they meet */
  int left = 0;
  int right = len - 1;
  while (left != right) {
    // Get the element on the left
    uint8_t left_byte = buf[left];
    // Store the element on the right. Probably necessary to store both, but
    // whatever.
    uint8_t right_byte = buf[right];

    // swap!
    buf[left] = right_byte;
    buf[right] = left_byte;

    left += 1;
    right -= 1;
  }
}

uint16_t sum_bytes(const uint8_t *buf, size_t len) {
  uint8_t total = 0;
  for (size_t i = 0; i < len; i++) {
    total += buf[i];
  }
  return total;
}
