fun main() {
    let a = true;
    const b = 2;
    let c = b == 2 && a;

    a = false;
    c = b == 2 && a;

    c(-(2 + 2), b);
}

/*


fun main() {
    let a = true;
    const b = 2;
    let c = b == 2 && a;

    a = false;
    c = b == 2 && a;

    c(-(2 + 2), b);
}


*/
