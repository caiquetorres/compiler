unsigned long long int fib(unsigned int n) {
  signed int a = 0;
  signed int b = 1;
  unsigned long long int c;
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
  fib(n);
}
