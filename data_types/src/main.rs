/* This code demonstares different types of data types and operations in Rust
    Scalar Types:
        - Integer - i8, i16, i32, i64, i128, u8, u16, u32, u64, u128 (Signed and Unsigned)
        - Floating Point - f32, f64
        - Boolean - bool (true, false) 
        - Character - char ('a', 'b', 'c') 
    Compound Types:
        - Tuple  => (1, 2, 3) 
        - Array  => [1, 2, 3] 
    Numeric Operation :
        - Addition  => 1 + 2    
        - Subtraction => 1 - 2
        - Multiplication => 1 * 2
        - Division => 1 / 2
        - Remainder => 1 % 2
        - Power => 1.pow(2)
        - Bitwise AND => 1 & 2
        - Bitwise OR => 1 | 2
        - Bitwise XOR => 1 ^ 2
        - Bitwise NOT => !1
        - Left Shift => 1 << 2
        - Right Shift => 1 >> 2
        - Logical AND => true && false
        - Logical OR => true || false
        - Logical NOT => !true
        - Comparison => 1 == 2, 1 != 2, 1 < 2, 1 > 2, 1 <= 2, 1 >= 2
        - Assignment => let x = 1
        - Compound Assignment => x += 1
        - Ternary Operator => let x = if true { 1 } else { 2 }
        - Type Conversion => let x: i32 = 1
        - Type Inference => let x = 1
        - Type Annotation => let x: i32
        - Type Alias => type MyInt = i32
        - Type Casting => let x = 1 as i32
        - Type Size => std::mem::size_of::<i32>()
        - Type Max Value => i32::MAX
        - Type Min Value => i32::MIN
        - Type Overflow => let x = i32::MAX + 1
        - Type Underflow => let x = i32::MIN - 1
        - Type Infinity => let x = f32::INFINITY
        - Type NaN => let x = f32::NAN
        - Type Zero => let x = 0.0f32
        - Type Negative Zero => let x = -0.0f32
*/

fn main() {
    let tup = (500, 6.4, 1);
    println!("Tuple: {:?}", tup); // Tuple: (500, 6.4, 1)
    println!("element 0: {}", tup.0); // element 0: 500
    println!("element 1: {}", tup.1); // element 1: 6.4
    println!("element 2: {}", tup.2); // element 2: 1

    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z); // x: 500, y: 6.4, z: 1
}
