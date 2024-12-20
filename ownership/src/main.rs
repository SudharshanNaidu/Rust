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

    Mutable References:
    -------------------
        We can have only one mutable reference to a particular piece of data in a particular scope.
        We can't have a mutable reference while we have an immutable reference.
        We can't have an immutable reference while we have a mutable reference.
        We can't have a mutable reference more than once in a particular scope.

    Dangling References:
    --------------------
        Rust ensures that references will never be dangling references.
        Rust's compiler will ensure that the references are always valid.

    Rules of References:
    --------------------
        1. At any given time, we can have either one mutable reference or any number of immutable references.
        2. References must always be valid.

    The Slice Type:
    ---------------
        Slices let us reference a contiguous sequence of elements in a collection rather than the whole collection.
        Slices are a reference to a part of a String.
        Slices are a reference to a part of a Vec.
        Slices are a reference to a part of an array.
        Slices are a reference to a part of a slice.

    String Literals:
    ----------------
        String literals are stored in the binary and are immutable

    String vs String Literal:
    -------------------------
        String:
            - A String is a growable, mutable, owned, UTF-8 encoded string type.
            - It is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
            - The String type is a wrapper over a Vec<u8> and is a growable, mutable, owned, UTF-8 encoded string type.
            - The String type is provided by Rust's standard library rather than coded into the core language.
            - Eg: let s = String::from("hello");

        String Literal:
            - String literals are stored in the binary and are immutable.
            - String literals are references to a part of the binary.
            - String literals are immutable references to a part of the binary.
            - Eg: let s = "hello";
       
    
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

    let s5 = String::from("hello");
    let len = calculate_length(s5);
    //println!("s5 : {}", s5); // This will throw an error as s5 is no longer valid. value borrowed here after move

    /* Referece and borrowing */
    let s6 = String::from("hello");
    let len = calculate_length_ref(&s6);
    println!("The length of '{}' is {}", s6, len); // This is valid as we are passing a reference to the function

    /* immutable reference */
    let s7 = String::from("hello");
    change_immute(&s7); //This will throw an error in change immute as we are trying to modify an immutable reference

    /* mutable reference */
    let mut s8 = String::from("hello");
    change_mut(&mut s8);
    println!("{}", s8);

    /* mutable reference and immutable reference */
    let mut s9 = String::from("hello");
    let r1 = &mut s9;
    //let r2 = &mut s9; // This will throw an error as we can't have more than one mutable reference in a particular scope

    let mut s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    //let r3 = &mut s10; // BIG NO NO. This will throw an error as we can't have a mutable reference while we have an immutable reference

    //println!("{}, {}, {}", r1, r2, r3);  

    /* Dangling Reference */
    //let reference_to_nothing = dangle(); // This will throw an error as we are trying to return a reference to a variable that goes out of scope

    /* Slices */
    let s11 = String::from("hello world");
    let hello = &s11[0..5];
    let world = &s11[6..11];
    println!("hello : {} world : {}", hello, world);

    let s12 = String::from("hello");
    let slice = &s12[..2];
    println!("slice : {}", slice);


}

/* 
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/ // This will throw an error as we are trying to return a reference to a variable that goes out of scope

fn change_mut(s: &mut String) {
    s.push_str(", world!");
    println!("{}", s);
}

fn change_immute(s: &String) {
    //s.push_str(", world!"); // This will throw an error as we are trying to modify an immutable reference
    println!("{}", s);

}

fn calculate_length_ref(st: &String) -> usize { // st is a reference to a String
    st.len() //st goes out of scope here, but because it does not have ownership of what it refers to, nothing happens.
}

fn calculate_length(s: String) -> usize {
    s.len()
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