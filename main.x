fun greet() {
    print("Hello world!");
}

fun main() {
    let counter: u8 = 0;

    while counter < 5.0 {
        print("While loop counter");
        counter += 1;

        if counter > 2 {
            break;
        }

        continue;

        for i in 0..5 {
            print("For loop counter");
            continue;
        }
    }

    greet();
}
