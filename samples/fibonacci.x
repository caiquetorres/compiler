fun main() {
    let fibs = [
        1,
        4,
        9
    ];

    for i in 0..3 {
        let result = fib(fibs[i]);
        println "Fib (", fibs[i], "): ", result;
    }
}

fun fib(n: i32) -> i32 {
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
