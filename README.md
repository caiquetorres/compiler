# Compiler

This repository contains a Rust compiler developed as part of a graduation project. The compiler is currently in development and is designed for creating our own programming language. The main goal of the project is to have fun while exploring compiler construction concepts.

## Current Status

We are actively working on building the foundational components of the compiler, including:

-   **Semantic Analyses**

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

Here's a snippet that the compiler totally can parse right now:

```x
fun main() {
    let counter = 0;

    while counter < 5 {
        print("While loop counter: {}", counter);
        counter += 1;
    }

    for i in 0..5 {
        print("For loop counter: {}", i);
    }

    // Comment
    greet("John");
}

/**
 * Documentation
 */
fun greet(language: str) {
    print("Hello, {}!", language);
}
```

## License

This project is licensed under the MIT License see the [LICENSE](LICENSE) file for details.
