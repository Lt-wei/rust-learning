//1. 基础要求：固定类型（比如i32）的数组排序
// fn bubble_sort(arr: &mut [i32]) {
//     let mut n = arr.len();
//     let mut swapped = true;

//     while swapped {
//         swapped = false;
//         for i in 1..n {
//             if arr[i - 1] > arr[i] {
//                 arr.swap(i - 1, i);
//                 swapped = true;
//             }
//         }
//         n -= 1;
//     }
// }

// fn main() {
//     let mut numbers = [4, 2, 7, 1, 3];
//     bubble_sort(&mut numbers);
//     println!("{:?}", numbers);
// }

// 2. 提高部分：能够使用范型和PartialOrd实现对任意类型的排序
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut n = arr.len();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }
}

fn main() {
    let mut numbers = [4, 2, 7, 1, 3];
    bubble_sort(&mut numbers);
    println!("{:?}", numbers);

    let mut float_numbers = [3.14, 2.71, 0.0, 4.20, 1.41];
    bubble_sort(&mut float_numbers);
    println!("{:?}", float_numbers);

    let mut words = ["banana", "apple", "pear", "orange"];
    bubble_sort(&mut words);
    println!("{:?}", words);
}
