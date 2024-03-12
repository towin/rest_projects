fn main() {
    // let x = 5;
    // let y = x;


    // let s1 = String::from("Hello");
    // let s2 = s1.clone();

    // println!("{}, world!", s1);

    let s = String::from("hello");

    takes_owership(s);
    // println!("{s}");

    let x = 5;

    makes_copy(x);
    println!("{x}");

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = take_and_give_back(s2);

    println!("{s1}");  
    // println!("{s2}");
    println!("{s3}");  


    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    print!("The length of '{}' is {}", s2, len);


}

fn takes_owership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let lenght = s.len();

    (s, lenght)
}