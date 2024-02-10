fn main() {
    println!("Hello, world!");

    another_function(123);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    // The value of y is: 4

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");

    // The value of x is: 5
    // The value of x is: 6
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}


fn plus_one(x: i32) -> i32 {
    x + 1
}