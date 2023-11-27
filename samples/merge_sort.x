fun main() {
    let array = [9, 8, 7, 5, 6, 4, 1, 2, 3, 0];

    println "The given array is: ";
    printArray(array);

    mergeSort(array, 0, 9);

    println;
    println "After sorting the result is: ";

    printArray(array);
}

fun mergeSort(array: [i32; 10], left: i32, right: i32) {
    if left < right {fua
        let mid = left + (right - left) / 2;

        mergeSort(array, left, mid);
        mergeSort(array, mid + 1, right);

        merge(array, left, mid, right);
    }
}

fun merge(array: [i32; 10], left: i32, mid: i32, right: i32) {
    let n1 = mid - left + 1;
    let n2 = right - mid;

    let l: [i32; 10];
    let r: [i32; 10];

    for i in 0..n1 {
        l[i] = array[left + i];
    }

    for j in 0..n2 {
        r[j] = array[mid + 1 + j];
    }

    let i = 0;
    let j = 0;
    let k = left;

    while i < n1 && j < n2 {
        if l[i] <= r[j] {
            array[k] = l[i];
            i += 1;
        } else {
            array[k] = r[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        array[k] = l[i];
        i += 1;
        k += 1;
    }

    while j < n2 {
        array[k] = r[j];
        j += 1;
        k += 1;
    }
}

fun printArray(array: [i32; 10]) {
    for i in 0..10 {
        print array[i];

        if i != 9 {
            print " ";
        }
    }
    println;
}
