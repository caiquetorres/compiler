# Compiler

This repository contains a Rust compiler developed as part of a graduation project. The compiler is currently in development and is designed for creating our own programming language. The main goal of the project is to have fun while exploring compiler construction concepts.

## Current Status

We are actively working on building the foundational components of the compiler, including:

-   **Code translation to C**
-   **Syntax Analyses**
    -   **Vectors and strings**
-   **Semantic Analyses**
    -   **Number implicit conversions**
    -   **Control flow graph**

Stay tuned for updates as we make progress on the project!

## Getting Started

To get started with the compiler, follow these steps:

1. [Step 1: Clone the repository](#step-1-clone-the-repository)
2. [Step 2: Build the compiler](#step-2-build-the-compiler)
3. [Step 3: Run the compiler](#step-3-run-the-compiler)

### Step 1: Clone the repository

Clone this repository to your local machine using:

```bash
git clone https://github.com/caiquetorres/compiler.git
```

### Step 2: Build the compiler

Navigate to the project directory and build the compiler using:

```bash
cd compiler
cargo build
```

### Step 3: Run the compiler

After building, you can run the compiler using:

```bash
cargo run -- --compile path/to/file
```

## Sneak a Peek at the Compiler 🚀

Here's a snippet that the compiler totally can compile right now:

```x
fun main() {
    let bin = 1101001;
    println "Result: ", convertToDecimal(bin);
}

fun convertToDecimal(number: u64): u32 {
    let n = number;
    let i = 0;
    let decimal = 0;

    while n > 0 {
        decimal += (n % 10) * pow(2, i);
        n /= 10;
        i += 1;
    }

    return decimal;
}

fun pow(base: i32, exponent: i32): i32 {
    if exponent == 0 {
        return 1;
    }

    if exponent % 2 == 0 {
        let halfPow = pow(base, exponent / 2);
        return halfPow * halfPow;
    }

    return base * pow(base, exponent - 1);
}
```

The compiler translates the code into C language. Below is an example demonstrating the converted code.

```c
#include <stdio.h>
unsigned int convertToDecimal(unsigned long long int);
signed int pow(signed int, signed int);
signed int main() {
  signed int bin = 1101001;
  printf("%s", "Result: ");
  printf("%u", convertToDecimal(bin));
  printf("\n");
  return 0;
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
```

## License

This project is licensed under the MIT License see the [LICENSE](LICENSE) file for details.
