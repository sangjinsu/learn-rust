const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let mut spaces = "   ";
    println!("{}", spaces);

    // same variable name, but different type
    // spaces = spaces.len();

    let spaces = spaces.len();
    println!("{}", spaces);

    println!("{}", THREE_HOURS_IN_SECONDS)
}