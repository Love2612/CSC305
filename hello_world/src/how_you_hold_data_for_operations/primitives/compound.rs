
// A tuple with a bunch of different types.
pub fn comp() {
let long_tuple = (1u8, 2u16, 3u32, 4u64,
    -1i8, -2i16, -3i32, -4i64,
    0.1f32, 0.2f64,
    'a', true);
// Values can be extracted from the tuple using tuple indexing.
println!("Long tuple first value: {}", long_tuple.0);
println!("Long tuple fifth value: {}", long_tuple.6);
println!("Long tuple second value: 0x{:x}", long_tuple.6);
}

pub fn comp2() {
    let tuple_of_tuples: ((u8, u16, i32), (u64, i8), i16) = ((43, 776, -9862), (466, -16), -2);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples); // to print everything in rust it has to be {:?}
}

fn reverse(pair: (&str, bool)) -> (bool, &str) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param) // there's no semicolon hence it tells the compiler to return
}

pub fn comp3() {
    let pair = ("red", false);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair)); 
}

use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn mayne() {
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500]; // 500 elements all containing the value of 0

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs)); //size of the array in the memory

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}


/*create a function, using rust language, named multiplier and it is 
supposed to burrow or take any array f64 and gives you the product of the array */

fn multiplier(numbers: &[f64]) -> f64 {
    let mut result: f64 = 1.0;
    for &num in numbers {
        result *= num;
    }
    result
}

pub fn maine() {
    let numbers = [12.0, 23.0, 54.0, 75.0];
    let product = multiplier(&numbers);
    println!("Product of the numbers: {}", product);
}


