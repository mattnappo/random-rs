fn succ(n: i32) -> i32 {
    n + 1
}

fn sum(a: &[i32]) -> i32 {
    let mut i = 0;
    let mut sum = 0;
    while i < a.len() {
        sum += a[i];
        i += 1;
    }
    sum
}

fn main() {
    let v = 8;
    let vsucc = succ(v);
    println!("succ({}) = {}", v, vsucc);

    let big = if vsucc > 10 { 999 } else { 9999 };
    println!("big = {}", big);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {} and counter is {}", result, counter);

    let array = [1, 2, 3, 4, 5];
    print!("array: ");
    for element in array.iter() {
        print!("{} ", element);
    }

    let s = sum(&array);
    println!("\nsum: {}", s);

    let mut literal = "im a literal";
    literal = " aghag hag hag hagh agh mutated!";
    println!("{}", literal);
}
