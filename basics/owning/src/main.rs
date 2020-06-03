fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("trying to use {}", s); // Doesn't work

    let mut e = 2.71828;
    e = makes_copy(e);
    println!("e is {}", e);

    let mut my_string = String::from("nice str");
    let my_string_ref = &mut my_string;
    my_string_ref.push_str(" NEW");
    println!("my string ref is {}", my_string_ref);
    println!("my string is {}", my_string);

    ////////

    let mut ss = String::from("nice");

    let ssr2 = &ss;

    let ssr1 = &mut ss;

    let mut new_s = "new literal";
    new_s = "modified new literal";
}

fn takes_ownership(string: String) {
    println!("took ownership of {}", string);
}

fn makes_copy(float: f32) -> f32 {
    println!("made a copy of {}", float);
    float + 1.0
}
