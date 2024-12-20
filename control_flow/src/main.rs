/*
    This example demonstrates the use of control flow in Rust.
    The control flow in Rust is similar to other programming languages.
    The control flow in Rust includes:
    1. if expressions
        if <condition> {
            // code block
        } else if <condition> {
            // code block
        } else {
            // code block
        }

    2. loop expressions
        loop {
            // code block
        }

        'label: loop {
            // code block
        }

    3. while expressions
        while <condition> {
            // code block
        }

    4. for expressions
        for <variable> in <iterator> {
            // code block
        }
        
    5. match expressions
        match <variable> {
            <pattern> => {
                // code block
            }
            <pattern> if <condition> => {
                // code block
            }
            _ => { //default case
                // code block
            }
        }
    6. continue
        continue is used to skip the current iteration of the loop. 

    7. break
        break is used to exit the loop. Also, break can be used to return a value from a loop.
        Eg: let result = loop {
            if <condition> {
                break <value>;
            }
        };

        breaking specific loop:
        'label: loop {
            loop {
                break 'label;
            }
        }

        multilevel break:
        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                if x == 1 && y == 1 {
                    break 'outer;
                }
            }
        }

    8. return
        return is used to return a value from a function.
        Eg: fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
*/

fn main() {
    let a = 6;

    /* demonstration of if conditions */
    if (a % 2) == 0 {
        println!("a is divisible by 2");
    } else if (a % 3) == 0 {  //expression in if condition should be a boolean unlike in c
        println!("a is divisible by 3");
    } else {
        println!("a is not divisible by 2 or 3");
    }

    /* Demonstration of loops */
    let mut a = 10;

    loop {
        println!("a: {}", a);
        if a == 3 {
            break;
        }
        a -= 1;
    }

    'outer: loop {
        'inner: loop {
            println!("inner loop");
            break 'outer; // breaking the outer loop
        }
    }

    let mut a = 10;

    while a >= 5 {
        println!("a: {}", a);
        a -= 1;
    }

    for i in (1..10) {
        println!("i: {}", i);
        if i == 5 {
            break;
        }
    }

    /* Demonstration of match expressions */
    let a = 3;
    match a {
        1 => println!("a is 1"),
        2 => println!("a is 2"),
        _ => println!("a is not 1 or 2"),
    }

}
