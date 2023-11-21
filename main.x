fun greet(): i32 {
    let a: i32 = 2;
    return 2.0;
}

fun main() {
    let counter: u8 = 0;

    while counter < 5.0 {
        counter += 1;

        if counter > 2 {
            break;
        }

        continue;

        for i in 0..5 {
            continue;
        }
    }

    greet();
}
