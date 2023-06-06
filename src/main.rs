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
fn main() {
    let first: Vec<i32> = vec![3, 4, 5, 8, 10, 15];
    let second: Vec<i32> = vec![1, 2, 6, 7, 8, 9];
    let final_vec = merge(first, second);
    println!("{:?}", final_vec);
}
