fun main() {
    let array = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
    let n = 10;

    let target = 10;
    let result = binarySearch(array, 0, n - 1, target);

    if result == -1 {
        println "Element is not present in the array";
    } else {
        println "Element is not present at index ", result;
    }
}

fun binarySearch(array: [i32; 10], left: i32, right: i32, target: i32): i32 {
    while left <= right {
        let mid = left + (right - left) / 2;

        if array[mid] == target {
            return mid;
        }

        if array[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}
