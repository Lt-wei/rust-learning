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
    let mut numbers = [5, 3, 8, 4, 2];
    bubble_sort(&mut numbers);
    println!("{:?}", numbers);

    let mut float_numbers = [5.5, 3.3, 8.8, 4.4, 2.2];
    bubble_sort(&mut float_numbers);
    println!("{:?}", float_numbers);

    let mut words = ["banana", "apple", "orange", "pear"];
    bubble_sort(&mut words);
    println!("{:?}", words);
}
