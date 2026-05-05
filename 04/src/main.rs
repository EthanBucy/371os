fn main() {
    let mut values = vec![1, 2, 3, 4, 5, 6];

    let (left, right) = hw_02_splits::split_at_mut(&mut values[..], 3);

    left[0] = 10;
    right[0] = 40;

    println!("left slice: {:?}", left);
    println!("right slice: {:?}", right);
    println!("final vector: {:?}", values);
}
