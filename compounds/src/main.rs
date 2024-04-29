use core::str;

fn main() {
    // arrays
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array numbers: {:?}", nums);

    let fruits: [&str; 3] = ["apple", "orange", "banana"];
    println!("Fruits array: {:?}", fruits);
    println!("First fruit: {}", fruits[0]);

    // tuples
    let human: (String, i32, bool) = ("Mario".to_string(), 49, true);
    println!("Mario = {:?}", human);

    let mix_tuple = (human, nums);
    println!("Mix = {:?}", mix_tuple);

    // slices
    let num_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slice = {:?}", num_slices);

    let animal_slices: &[&str] = &["dog", "cat", "lion"];
    println!("Animal slice = {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"Book 1".to_string(),
        &"Book 2".to_string(),
        &"Book 3".to_string(),
    ];
    println!("Book slice = {:?}", book_slices);

    // String is mutable
    let mut name: String = String::from("Mario");
    name.push_str(" Lazzari");
    println!("My name is {}", name);

    // String slice
    let string: String = String::from("My string");
    let slice: &str = &string;
    println!("Slice value is {}", slice);

    let slice_str: &str = &string[0..3];
    println!("First 3 chars: {}", slice_str);
}
