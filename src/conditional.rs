pub fn conditional(num1: i8, num2: i8 ) {
    if num1 > num2 {
        println!("{} Greater than {}", num2, num1);
    }
    else if num1 < num2 {
        println!("{} Greater than {} ", num2, num1);
    }
    else {
        println!("is equal");
    }
}