fun multiply(a: i32, b: i32): i32 {
    return a * b;
}

fun main(): i32 {
    let num1: i32 = -(8 + 2) * 3;
    let num2 = 4;

    while num1 == 2 {
        print(num1);
    }

    if num1 == num2 {
        print(num1);
    } else {
        print(num2);
    }

    return multiply(num1, num2);
}
