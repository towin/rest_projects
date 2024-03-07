fn main() {
    //Scalar Types 标量
    //Integer
    let int_x = 55;
    println!("Here is a Integer: int_x = {int_x}");

    //Floating-Point Types
    let x = 2.0; //f64
    let y: f32 = 3.0; // f32

    //Numeric Operations
    let sum = 5 + 10;
    let difference  = 95.5 - 4.3;
    let product = 4 * 3;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    println!("quotient is: {quotient}");
    println!("truncated is: {truncated}");

    //The Boolean Type
    let t = true;
    let f: bool = false;

    //The Character
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Here are three chars: {}, {}, {}", c, z, heart_eyed_cat);



    //Compound Types
    //The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The second element of the tuple is: {}", tup.1);

    //The Array Type
    let a = [1, 2, 3, 4, 5];
}
