fn main() {
    let mut s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;

    println!("{}, {}", r1, r2);
    let r3 = &mut s1;




    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    chage(&mut s1);

    let r = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn chage(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}