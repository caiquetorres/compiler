/**
 * Documentation
 */
fun greet() {
    print("Hello world!");
}

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
    greet();
}
