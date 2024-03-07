fn main() {
    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1

    };
    println!("The value of x is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(x);
    println!("The value of x is: {x}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}