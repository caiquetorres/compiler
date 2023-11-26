fun main() {
    let bins = [
        100,
        1001,
        1101001
    ];

    for i in 0..3 {
        let j = i + 0;
        let result = convertToDecimal(bins[j]);
        println "Bin (", bins[j], ") to decimal ", result;
    }
}

fun convertToDecimal(number: u64) -> u32 {
    let n = number;
    let i = 0;
    let decimal = 0;

    while n > 0 {
        decimal += (n % 10) * pow(2, i);
        n /= 10;
        i += 1;
    }

    return decimal;
}

fun pow(base: i32, exponent: i32) -> i32 {
    if exponent == 0 {
        return 1;
    }

    if exponent % 2 == 0 {
        let halfPow = pow(base, exponent / 2);
        return halfPow * halfPow;
    }

    return base * pow(base, exponent - 1);
}
