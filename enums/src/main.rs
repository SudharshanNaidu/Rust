/*
    This example demonstrates how to use enums in Rust.
    Enums are a way to create a type by grouping together a set of variants.
    Enums are useful when you have a fixed set of values that you know about at compile time.

    Enums are defined using the enum keyword and the variants are separated by commas.
    Each variant can optionally have data associated with it.

    Enums can be used in match expressions to match against the different variants.
    Enums can also be used in structs to create a type that can have one of a fixed set of values.

*/
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum Coin {
    quarter,
    half,
    rupee,
    two_rupee,
    five_rupee,
}

fn value_of_coin(coin: Coin) -> f32 {
    match coin {
        Coin::quarter => 0.25,
        Coin::half => 0.5,
        Coin::rupee => 1.0,
        Coin::two_rupee => 2.0,
        Coin::five_rupee => 5.0,
    }
}

fn main() {
    let coin = Coin::five_rupee;
    println!("coin value is {}", value_of_coin(coin));

    let coin = Coin::quarter;
    println!("coin value is {}", value_of_coin(coin));
}
