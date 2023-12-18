struct Example;

fn main() {
    Example::print_hello();
    println!("forty_two: {}", Example::forty_two());
    let num1 = 42;
    let num2 = 33;
    println!("{} equals 42 is {}", num1, Example::forty_two_validator(num1));
    println!("{} equals 42 is {}", num2, Example::forty_two_validator(num2));
}

impl Example {
    fn print_hello() {
        println!("Hello!");
    }
    
    fn forty_two() -> i32 {
        42
    }

    fn forty_two_validator(num: i32) -> bool {
        if num == 42
        {
            true;
        }
        false
    }
}
