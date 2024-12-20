fn main() {
    /*---- Variables---- */

    let mut x = 5; //Defaukt data type is i32
    println!("The value of x is {x}");

    x = 6; //Value 6 can't be assigned if x is not declared with mut
    println!("The value of x is {x}");

    /*---- Constans---- */
    const PI: f32 = 22.0 / 7.0;  // Constants always declare with data type (f32 here)
    println!("The value of PI is {PI}");

    /*---- Shadowing---- */
    let a = 10;
    let a = 10 * a; // Variable a is shadowed

    {
        let a = 10 * a;
        println!("The value of a inside is {a}"); //The value of a inside is 1000
    }

    println!("The value of a outside is {a}"); //The value of a outside is 100

    let spaces = "    ";
    let spaces = spaces.len(); //Shadow variable can be converted to different types: here from string to integer

    println!("The value of spaces is {spaces}");


}
