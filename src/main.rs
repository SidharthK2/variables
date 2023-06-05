fn main() {
    let s = String::from("Hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("{x}");
}

fn takes_ownership(some_string: String) {
    println!("from fn {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("from fn {some_integer}");
}
