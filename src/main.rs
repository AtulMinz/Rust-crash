//importing from other members

mod conditional;
mod loops;
mod slice;
mod strings;
mod structures;
mod switch;
use conditional::conditional;
use loops::loops;
use slice::slice;
use strings::strings;
use structures::Bird;
use switch::switch;

fn main() {
    // We use underscore before unused variable just to prevent the unused warning.

    //signed integer
    let _a: i8 = 7;
    //unsigned integer
    let _b: u8 = 14;
    //emoji
    let _emoji = "\u{1F600}";
    //Arrays
    let _arr: [u8; 3] = [1, 2, 3];
    let _other_arr: [u8; 5] = [7; 5]; //repeats the number 7 5 times in an array.
                                      //Tuples
    let tuple: (u8, bool, f32) = (7, true, 7.1);
    let _tuple2 = (14, 25);
    let (_a, _b, _c) = tuple; //destructing a tuple.

    //functions
    pub fn is_even(num: u8) -> bool {
        let digit: u8 = num % 2;
        digit == 0 //return boolean
    }

    //mutability
    let mut _num: i8 = 5;
    _num = 7;

    //Slice
    let nums: &[i8] = &[0, 1, 2, 3, 4];
    slice(&nums);

    //Strings
    let str: &str = "hello world";
    strings(str);

    //Conditional Statements
    conditional(7, 7);

    //Loops
    let n: i8 = 5;
    loops(n);

    //switch
    switch();

    //Structures
    let name = String::from("Pigioto");
    let bird = Bird { name, attack: 20 };
    bird.print_name();

    //Print statements
    // println!("Hello, world! {} {} {}", a, b, emoji);
    // print!("Array: {:?} {}", arr, arr.len());
    // println!("Other Array: {:?}", other_arr);
    // println!("Tuple {} {}", _tuple2.0, _tuple2.1);
    // println!("De-structured Tuple {} {} {}", a, b, c);
    println!("{}", is_even(8));
    println!("Mutated {}", _num);
}
