pub fn loops(_n: i8) {
    // for i in 0..n {
    //     println!("{}", i);
    // }

    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i += 1;
        if i == 3 {
            println!("exit");
            break;
        }
    }
}