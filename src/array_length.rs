fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr1: [i32; 2] = [4, 5];
    display_array(arr1);
}