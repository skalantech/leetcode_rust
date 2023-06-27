#![allow(unused)]

use std::mem;

// fn swap(a: &mut i32, b: &mut i32) {
//     let tmp = *a;
//     *a = *b;
//     *b = tmp;
// }

fn bubble_sort(a: &mut [i32]) {
    let size = a.len();
    for i in 0..size {
        for j in 0..size - i - 1 {
            if a[j] > a[j + 1] {
                a.swap(j, j+1)
            }
        }
    }
}

fn selection_sort(a: &mut [i32]) {
    let size = a.len();
    for i in 0..size - 1 {
        let mut min_index = i;
        for j in i + 1..size {
            if a[j] < a[min_index] {
                min_index = j;
            }
        }
        a.swap(min_index, i);
    }
}

fn insertion_sort(a: &mut [i32]) {
    let size = a.len() as i32;
    for i in 1..size {
        let key = a[i as usize];
        let mut j = i - 1;
        while j >= 0 && key < a[j as usize] {
            a[(j + 1) as usize] = a[j as usize];
            j -= 1;
        }
        a[(j + 1) as usize] = key;
    }
}

fn merge(a: &mut [i32], index_zero: usize, index_middle: usize, size: usize) {
    let p = index_zero;
    let q = index_middle;
    let r = size;

    let n1 = q - p + 1;
    let n2 = r - q;

    let mut l = Vec::new();
    let mut m  = Vec::new();

    for i in 0..n1 {
        l.push(a[p + i]);
    }
    for j in 0..n2 {
        m.push(a[q + 1 + j]);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < n1 && j < n2 {
        if l[i] <= m[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = m[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        a[k] = l[i];
        i += 1;
        k += 1;
    }

    while j < n2 {
        a[k] = m[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort(a: &mut [i32], index_zero: usize, size: usize) {
    let l = index_zero;
    let r = size;
    if l < r {
        let m = l + (r - l) / 2;

        merge_sort(a, l, m);
        merge_sort(a, m + 1, r);

        merge(a, l, m, r);
    }
}

fn print_array(a: &[i32]) {
    for i in 0..a.len() {
        print!("{} ", a[i]);
    }
    println!();
}

pub fn main() {
    let mut data0 = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut data1 = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut data2 = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut data3 = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    let size = data0.len();
    println!("{}", size);

    bubble_sort(&mut data0);
    print_array(&data0);

    selection_sort(&mut data1);
    print_array(&data1);

    insertion_sort(&mut data2);
    print_array(&data2);
    
    merge_sort(&mut data3, 0, size - 1);
    print_array(&data3);
}

