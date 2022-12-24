use std::io::stdin;

fn main() {

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error getting user input");
    let input: u64 = input.trim().parse().expect("Error parsing input");
    
    println!("{}", fibonacci(input));
}



fn fibonacci(number:u64) -> u64 {
    if number == 0 {
        0
    } else if number == 1 {
        1
    } else {
        fibonacci(number - 1) + fibonacci(number - 2)
    }
}
