pub fn strings(_str: &str) {
    let mut string: String = String::from("Hello World");

    let slice = &string[.. 5];
    println!("Length: {}",slice.len());

    string.push('!');
    string.push_str("faith");
    string = string.replace("Hello", "Hi");
    println!("{}", string);
}