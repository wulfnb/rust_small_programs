use std::io::stdin;
mod my_calculator;

fn main() {
    println!("Please enter the first number");
    let first_number = get_input();
    println!("Please enter the seccond number");
    let second_number = get_input();
    let mut result = 0;

    println!("Please enter what operation needs to do\n1.Add\n2.Substact\n3.Multiply\n4.Device\n");
    let choice = get_input();
    match choice{
        1=> result = my_calculator::add(first_number, second_number),
        2=> result = my_calculator::substarct(first_number, second_number),
        3=>result = my_calculator::multiply(first_number, second_number),
        4=>result = my_calculator::devide(first_number, second_number),
        _=>println!("Not a valid input"),   
    }

    println!("The result is {}", result);
}

fn get_input() -> i32{
    let mut line  = String::new();
    stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}