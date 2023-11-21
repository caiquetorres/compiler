# Compiler

This repository contains a Rust compiler developed as part of a graduation project. The compiler is currently in development and is designed for creating our own programming language. The main goal of the project is to have fun while exploring compiler construction concepts.

## Current Status

We are actively working on building the foundational components of the compiler, including:

-   **Semantic Analyses**
    -   **Number implicit conversions**
-   **Code conversion**

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
/**
  * Fibonnaci function
  */
fun fib(n: u32): u64 {
    let a = 0;
    let b = 1;
    let c: u64;

    if n == 0 {
        return a;
    }

    for i in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }

    return b;
}

fun main() {
    let n = 9;
    fib(n);

    // Print result
}
```

The compiler translates the code into C language. Below is an example demonstrating the converted code.

```c
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
```

## License

This project is licensed under the MIT License see the [LICENSE](LICENSE) file for details.
