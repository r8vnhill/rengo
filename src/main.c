#include <stdio.h>
#include <stdint.h>

extern int64_t _start() asm("_start");

int main(int argc, char** argv) {
  int64_t result = _start();
  printf("%lld\n", result);
  return 0;
}
