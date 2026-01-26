fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!")
    } else {
        println!("{seconds} seconds to blastoff...");
        countdown(seconds - 1);
    }
}

use std::cmp::PartialOrd;
fn merge<T: PartialOrd + Copy + std::fmt::Debug>(data: &mut [T], mid: usize, is_debug: bool) -> () {
    // temp array to store data
    let mut temp: Vec<T> = Vec::with_capacity(data.len());

    // left and right arrays
    let (left, right): (&[T], &[T]) = data.split_at(mid);

    if is_debug {
        println!("  Merging: {:?} and {:?}", left, right);
    }

    let mut i: usize = 0; // left pointer
    let mut j: usize = 0; // right pointer

    // loop through
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            temp.push(left[i]);
            i += 1;
        } else {
            temp.push(right[j]);
            j += 1
        }
    }

    // copy remaining elements from left
    temp.extend_from_slice(&left[i..]);
    temp.extend_from_slice(&right[j..]);

    // copy the data from temp to well, data
    data.copy_from_slice(&temp);

    if is_debug {
        println!("  Result:  {:?}", data);
    }
}

// that is a generic type
// so we want to be able to sort string as well
// so we stated that this T uses the following traits
// we also need std::fmt::Debug for printing
fn merge_sort<T: PartialOrd + Copy + std::fmt::Debug>(data: &mut [T], is_debug: bool) -> () {
    let len: usize = data.len();
    if len < 2 {
        return;
    }

    if is_debug {
        println!("Splitting: {:?}", data);
    }

    // mid point
    let mid: usize = len / 2;
    merge_sort(&mut data[..mid], is_debug);
    merge_sort(&mut data[mid..], is_debug);
    merge(data, mid, is_debug);
}

fn main() {
    // basically a function that calls itself
    // nothing magical
    // just practical
    countdown(5);

    // recursive algorihm
    // merge sort, divide and conquer algorithm
    // it's alsot recursive
    println!();
    let mut test_data: Vec<i32> = vec![-12, -50, 43, 123, 433, -332, 0, 564, 1220, 331];
    println!("Data before sorting: {:?}", test_data);
    merge_sort(&mut test_data, true);
    println!("Data after sorting: {:?}", test_data);
}
