/*
    This demonstrates the ownership concept in Rust.
    Ownership is a unique feature of Rust that makes it different from other languages.
    It is a way of managing memory and resources in Rust.
    Ownership Ruls:
    1. Each value in Rust has a variable that is called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.

    The String type:
    ----------------
        The String type is a growable, mutable, owned, UTF-8 encoded string type.
        It is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
        The String type is provided by Rust's standard library rather than coded into the core language.
        The String type is a wrapper over a Vec<u8> and is a growable, mutable, owned, UTF-8 encoded string type.

        Eg: let s = String::from("hello");

    Memory and Allocation:
    ----------------------
        In Rust, the memory is automatically returned once the variable that owns it goes out of scope.

    Copy trait:
    -----------
        Rust has a special annotation called the Copy trait that we can place on types like integers that are stored on the stack.
        If a type has the Copy trait, an older variable is still usable after assignment.
        Rust won't let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait.
        If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we'll get a compile-time error.  

    Ownership and Functions:
    ------------------------
        Passing a variable to a function will move or copy, just as assignment does.
        When we pass a variable to a function, the ownership of the variable is transferred to the function.
        If we want to use the variable after passing it to a function, we can return the ownership of the variable to the calling function.

    Return Values and Scope:
    ------------------------
        Returning values can also transfer ownership.
        When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it is where the author of String can put the code to return the memory.
        Rust calls drop automatically at the closing curly brace.     

    References and Borrowing:
    -------------------------
        We can use references to pass a variable to a function without transferring ownership.
        References allow us to refer to some value without taking ownership of it.
        We can create a reference by using the & symbol.
        We can pass a reference to a function to borrow the value.
        We can have only one mutable reference to a particular piece of data in a particular scope.
        We can have any number of immutable references to a particular piece of data in a particular scope.
        References must always be valid.

    
*/

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    //let mut s1 = s;
    //s.push_str(" This is a test"); // This will throw an error as s is no longer valid. value borrowed here after move

    let s1 = s.clone(); // This will create a deep copy of s and assign it to s1
    s.push_str(" This is a test"); // This is valid due to deep copy of s to s1
    println!("s : {} s1 : {}", s, s1);

    let x = 5;
    let st = String::from("hello");

    takes_ownership(st);
    makes_copy(x);
    
    println!("x : {}", x); // This is valid as x is an integer and has the Copy trait
    //println!("st : {}", st); // This will throw an error as st is no longer valid. value borrowed here after move

    let s2 = gives_ownership();
    println!("s2 : {}", s2);

    let s3 = String::from("hello");
    let s4 = takes_and_gives_back(s3);

    //println!("s3 : {}", s3); // This will throw an error as s3 is no longer valid. value borrowed here after move
    println!("s4 : {}", s4);


}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}