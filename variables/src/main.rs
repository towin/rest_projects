fn main() {
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("======================================");

//    const THERE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the scope is: {}", x);
    }

    println!("The value of x is: {x}");


    let mut spaces = "           ";
    spaces = spaces.len();

    println!("The value of spaces is: {spaces}");
}
