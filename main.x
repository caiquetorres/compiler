/**
 * Documentation
 */
fun greet() {
    print("Hello world!");
}

fun main() {
    const counter: u8 = 0;

    while counter < 5.0 {
        print("While loop counter");
        counter += 1;
    }

    for i in 0..5 {
        print("For loop counter");
    }

    // Comment
    greet();
}
