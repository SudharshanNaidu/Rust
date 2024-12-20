/*
    ---------------Funcions in Rust----------------
    Functions in Rust are declared using the fn keyword. Here is an example of a function that takes no arguments and returns nothing:
    fn test() {
        println!("This is a test function");
    }
    
    Functions can take arguments and return values. Here is an example of a function that takes two arguments and returns the sum of the two arguments:
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    Functions can also be generic, meaning they can take arguments of any type. Here is an example of a generic function that takes two arguments of any type and returns the sum of the two arguments:
    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    Functions can also be recursive, meaning they can call themselves. Here is an example of a recursive function that calculates the factorial of a number:
    fn factorial(n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    Functions can also be closures, meaning they can capture variables from their environment. Here is an example of a closure that captures a variable from its environment:
    fn make_adder(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }

    Functions can also be passed as arguments to other functions. Here is an example of a function that takes a function as an argument and calls it:
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }

    Functions can also be returned from other functions. Here is an example of a function that returns a function:
    fn make_adder(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }

    Functions can also be stored in data structures. Here is an example of a vector of functions:
    let functions = vec![
        |x| x + 1,
        |x| x * 2,
        |x| x - 1,
    ];
*/

fn main() {
    println!("in main function");
    another_function(); //function without arguments
    function_with_arguments(5, 6); //function with arguments
    let x = function_with_return_value(); //function with return value
    let y = function_with_return_value_and_arguments(5, 6); //function with return value and arguments
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    //function with generic arguments
    //println!("The value of {x} + {y} is: {result}", x = 5, y = 6, result = addition(5, 6));

    let (a, b) = function_with_multiple_return_values(25, 20);

    println!("The value of a is: {} and the value of b is: {}", a, b);
}

/*
fn addition(a: T, b: T) -> T {
    a + b
}
*/

fn function_with_multiple_return_values(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}


fn function_with_arguments(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn function_with_return_value() -> i32 {
    6 + 6 //returning the sum of 6 and 6 without using semicolon
    //return 6 + 6; //returning the sum of 6 and 6 using return keyword
}

fn function_with_return_value_and_arguments(x: i32, y: i32) -> i32 {
    return x * y; //returning the product of x and y using return keyword  
}

fn another_function() {
    println!("Another function.");
}
