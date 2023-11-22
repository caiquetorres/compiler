fun fib(n: i32): i32 {
    let a = 0;
    let b = 1;
    let c: i32;

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
    println "Result: ", fib(n);
}
