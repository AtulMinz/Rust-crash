pub fn switch() {
    let i = 4;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1, 2"),
        3..=5 => println!("middle"),
        _ => println!("default")
    }
}
