// src/how_you_hold_data_for_operations/primitive.rs

// Boolan operation
pub fn perform_operation1() -> bool {
    let bool = false;

    return bool;
}

// signed integer operation
pub fn perform_operation2() -> i32 {
    let x: i32 = -65;
    let y: i32 = -55;
    return x + y;
}


//float operation
pub fn perform_operation3() -> f32 {
    let a: f32 = 64.76;
    let b: f32 = 32.88;

    return a - b;
}

// unsigned integer operation
pub fn perform_operation4() -> u32 {
    let s: u32 = 65;
    let t: u32 = 55;

    return s + t;
}

//String
pub fn perform_operation() -> String {
    let message = String::from("I am so tired");
    message
} 

pub fn perform_operation5() {
    // Integer addition
    //Literals and operations
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3); //e4 is 10 raised to 4 and e-3 is 10 raised to -3 hence e is 10

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);//04b is 4 integers and should be printed in binary (bitwise)
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5); // this is left shift 5 times hence it would be 100000 binary but 32 in decimal
    println!("1 >> 5 is {}", 1u32 >> 5); // this is right shift
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); //this is right shift a hexadecimal 2 times i.e 0x80 is 10000000 but when right shifted it is 00100000 which is 20 in hexadecimal (ox20)
    // and {:x} shows that it would convert to hexadecimal but it is easier to understand when you put the string 0x before the braces

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
    println!("One million is written as {}", 1_000_000u32 as f32 - 2.5);
}
