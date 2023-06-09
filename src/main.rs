use std::io;
fn merge(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    let mut final_vec: Vec<i32> = vec![];
    let mut first_pointer: usize = 0;
    let mut second_pointer: usize = 0;
    while first_pointer <= first.len() - 1 && second_pointer <= second.len() - 1 {
        if first[first_pointer] > second[second_pointer] {
            final_vec.push(second[second_pointer]);
            second_pointer += 1;
        } else {
            final_vec.push(first[first_pointer]);
            first_pointer += 1;
        }
    }
    if first_pointer <= first.len() {
        for i in first_pointer..first.len() {
            final_vec.push(first[i]);
        }
    }

    if second_pointer <= second.len() {
        for i in second_pointer..second.len() {
            final_vec.push(second[i]);
        }
    }
    return final_vec;
}
fn split(array: Vec<i32>) -> Vec<Vec<i32>> {
    let mut vector_of_array: Vec<Vec<i32>> = vec![];
    for i in 0..array.len() {
        vector_of_array.push(vec![array[i]]);
    }
    return vector_of_array;
}
fn sort(mut array: Vec<Vec<i32>>) -> Vec<i32> {
    if array.len() == 1 {
        let sorted_array: Vec<i32> = array[0].clone();
        return sorted_array;
    }
    let mut pointer: usize = 0;
    while pointer < array.len() - 1 {
        let first: Vec<i32> = array[pointer].clone();
        let second: Vec<i32> = array[pointer + 1].clone();
        array[pointer] = merge(first, second);
        array.remove(pointer + 1);
        pointer += 2;
    }

    sort(array)
}
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let numbers: Vec<_> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Invalid number"))
        .collect();
    let sorted_vec = split(numbers);
    let final_vec = sort(sorted_vec);
    println!("{:?}", final_vec);
}
