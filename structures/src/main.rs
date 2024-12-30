/*
    This example demonstrates the use of structures in Rust
    A structure is a way to group multiple variables together into a single unit.
    Structures are similar to tuples.
    Structures are similar to dictionaries in Python.

    Syntax:
    struct Name {
        field1: type1,
        field2: type2,
        field3: type3,
        ...
    }

    Example:
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    We can create an instance of a structure by using the struct's name and curly braces.
    We can access the fields of a structure by using the dot notation.

    Example:
    let user1 = User {
        username: String::from("user1"),
        email: String::from("  

    
    We can use the dot notation to access the fields of a structure.

    Example:
    let user1 = User {
        username: String::from("user1"),
        email: String::from("

    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);

    
*/

/* Template */

struct User {
    name: String,
    age: u8,
    mail: String,
}

fn build_user() -> User {
    User {
        name: String::from("abc"),
        age: 40,
        mail: String::from("abc@mail.com"),
    }

    /* This is an expression */
}

fn build_user2(name: String, mail: String) -> User {
    User {
        name, //alternatively name: name
        age:50,
        mail, //alternatively mail: mail
    }
}

#[derive(Debug)]  // This is to print whole structure in println!("Dimensions of rect are {:?}", rect1); 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = User {
        name: String::from("john"),
        age: 32,
        mail: String::from("john@mail.com"),
    };

    println!("{}", user1.name);
    println!("{}", user1.age);
    println!("{}", user1.mail);

    let user2 = build_user();

    println!("{}", user2.name);
    println!("{}", user2.age);
    println!("{}", user2.mail);

    let user3 = build_user2("def".to_string(), "def@mail.com".to_string());

    println!("{}", user3.name);
    println!("{}", user3.age);
    println!("{}", user3.mail);

    let user4 = User {
        mail: String::from("xyz@mail.com"),
        ..user1 // This moves all the data from user1 except mail
    };

    println!("{}", user4.name);
    println!("{}", user4.age);
    println!("{}", user4.mail);

    //println!("{}", user1.name); // This causes error since value of name moved to user 4
    println!("{}", user1.age);
    println!("{}", user1.mail);

    /* Struct borrow */
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    /* The below lines are not valid since the values are moved
            let area:u32 = area_rect(rect1);
        println!("Area of {} * {} rectangle is {}", rect1.height, rect1.width, area);

        changes fn area_rect(rect: Rectangle) -> u32  {}
        to fn area_rect(rect: &Rectangle) -> u32 {
     */
    let area:u32 = area_rect(&rect1);
    println!("Area of {} * {} rectangle is {}", rect1.height, rect1.width, area);
    println!("Dimensions of rect are {:?}", rect1);

    dbg!(&rect1);
}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
