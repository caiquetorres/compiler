fun main() {
    let str = "amanaplanacanalpanama";
    let size = 21;

    if isPalindrome(str, size) {
        println "The given string is a palindrome.";
    } else {
        println "The given string is not a palindrome";
    }
}

fun isPalindrome(str: string, size: i32): bool {
    let left = 0;
    let right = size - 1;

    while right > left {
        if str[left] != str[right] {
            return false;
        }

        left += 1;
        right -= 1;
    }

    return true;
}
