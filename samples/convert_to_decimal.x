fun pow(base: i32, exponent: i32): i32 {
    if exponent == 0 {
        return 1;
    }

    if exponent % 2 == 0 {
        let halfPow = pow(base, exponent / 2);
        return halfPow * halfPow;
    }

    return base * pow(base, exponent - 1);
}

fun convertToDecimal(number: u64): u32 {
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

fun main() {
    let bin1 = 100;
    let bin2 = 1001;
    let bin3 = 1101001;

    let result1 = convertToDecimal(bin1);
    let result2 = convertToDecimal(bin2);
    let result3 = convertToDecimal(bin3);

    println "Bin 1 to decimal ", result1;
    println "Bin 2 to decimal ", result2;
    println "Bin 3 to decimal ", result3;
}
