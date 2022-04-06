fn main() {
    let s = String::from("hello world!");

    println!("{}", first_word(&s));
    println!("{}", type_of([1, 2, 3]));
    println!("{}", type_of("hello".as_bytes().iter()));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &element) in bytes.iter().enumerate() {
        if element == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[allow(dead_code)]
fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    a.to_string()
}
