#include <stdio.h>
signed int pow(signed int base, signed int exponent) {
  if (exponent == 0) {
    return 1;
  }
  if (exponent % 2 == 0) {
    signed int halfPow = pow(base, exponent / 2);
    return halfPow * halfPow;
  }
  return base * pow(base, exponent - 1);
}
unsigned int convertToDecimal(unsigned long long int number) {
  unsigned long long int n = number;
  signed int i = 0;
  signed int decimal = 0;
  while (n > 0) {
    decimal += (n % 10) * pow(2, i);
    n /= 10;
    i += 1;
  }
  return decimal;
}
signed int main() {
  signed int bin = 1101001;
  printf("%s", "Result: ");
  printf("%u", convertToDecimal(bin));
  printf("\n");
  return 0;
}
