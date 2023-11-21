#include <stdio.h>
signed int fib(signed int n) {
  signed int a = 0;
  signed int b = 1;
  signed int c;
  if (n == 0) {
    return a;
  }
  int i;
  for (i = 2; i <= n; i++) {
    c = a + b;
    a = b;
    b = c;
  }
  return b;
}
void main() {
  signed int n = 9;
  signed int result = fib(n);
  printf("%s", "Result ");
  printf("%d", result);
  printf("\n");
}
