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
    let fib1 = 1;
    let fib2 = 4;
    let fib3 = 9;

    let result1 = fib(fib1);
    let result2 = fib(fib2);
    let result3 = fib(fib3);

    println "Fib 1", result1;
    println "Fib 2", result2;
    println "Fib 3", result3;
}
