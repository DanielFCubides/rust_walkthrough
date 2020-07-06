// The Rules of References
//
// Let’s recap what we’ve discussed about references:
//
//     At any given time, you can have either one mutable reference or any number of immutable references.
//     References must always be valid.


fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}