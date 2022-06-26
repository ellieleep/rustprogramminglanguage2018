use std::io;

fn main(){
    println!("What to convert");
    println!("1. Centigrade to Fahrenheit");
    println!("2. Fahrenheit to Centigrade");

    let mut selection = String::new();

    io::stdin().read_line(&mut selection).expect("Failed to read line");

    let selection:i32 = selection.trim().parse().expect("Please type a number!");

    println!("Please enter a temperature to convert");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let temp:f32 = temp.trim().parse::<f32>().expect("Please type a number");

    if selection == 1 { println!("{}C to Fahrenheit is {}F", temp, (temp*(1.8)+32.0)) }
    else if selection == 2 { println!("{}F to Celsius is {}C", temp, ((temp-32.0)*(5.0/9.0))) }
}