fn main() {
    let s = String::from("Welcome to Anzus Corporation");
    let len = s.len();

    let slice1 = &s[..15];
    let slice2 = &s[15..];


    println!("string: {}, with len {}",s,len);
    println!("slice1: {}, slice2 {}",slice1,slice2);
    println!("first word : {}", first_word(&s));


    let a = [1, 2, 3, 4, 5];
    let slicea = &a[1..3];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
