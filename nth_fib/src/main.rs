use std::io;

fn main(){
    println!("Enter fib sequence number");

    let mut fib_number = String::new();

    io::stdin().read_line(&mut fib_number).expect("Failed to read line");

    let fib_number:u32 = fib_number.trim().parse().expect("Please type a number");

    let mut fib_total:u128 = 1;

    let mut total_previous_one = 0;

   for _count in 1..fib_number {
       let total_previous_two = total_previous_one;
       total_previous_one = fib_total;
       fib_total = total_previous_two + total_previous_one;
   }
    println!("fib sequence number {} is {}", fib_number, fib_total);
}