
fn compare_stuff() {
    let x = 1;
    let x_str = "1".to_string();
    match x.to_string().cmp(&x_str) {
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
        Ordering::Equal => println!("equal"),
    }

    match x.to_string().cmp(&x_str) {
        Ordering::Less | Ordering::Greater => println!("not equal"),
        Ordering::Equal => println!("equal"),
    }

    let number = 13;
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }
}