#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

struct Slice {
  int32_t *ptr;
  uint64_t len;
};

struct Slice wrapper(void);

int main() {
  struct Slice s = wrapper();
  for (int i = 0; i < s.len; i++) {
    printf("s[%d] = %d\n", i, s.ptr[i]);
  }
  return 0;
}
