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
