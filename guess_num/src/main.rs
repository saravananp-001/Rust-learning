use std::io;
use rand::Rng;
// use rand::thread_rng;
use std::cmp::Ordering;

fn main() {

    println!("generating secret number");
    
    //so no need rand::thread_rng()
    // gen_range() => rand::Rng
    let secret_number = rand::thread_rng().gen_range(1..101);   

    println!("your secret number was {}",secret_number);

    println!("Guess the number");
    println!("Enter the guess number");
     
    //adding loop for the correct guess
    loop{
          // initializing the number with mut for changeing the value
          let mut number = String::new(); 

          io::stdin()
              .read_line(&mut number)         // reading the line  (&reference must need)
              .expect("Failed to read line"); // Handling the Result from read_line()
          
          // converting into to string
          // number = 10.to_string();             
          print!("your guess number is {}",number);
          
          //changing the String to u32 data type 
        //   let num: u32 = number.trim().parse().expect("Please type a number!");
          
          // way to handle expect to avoid crash of 
          // "thread 'main' panicked at 'Please type a number!: ParseIntError"
          let num: u32 = match number.trim().parse(){
              Ok(num) =>num,       // here this num asign to -> let num
              Err(_) =>continue,  // (_)means no mater whats inside the error
          };

          // if condition
          if num < secret_number {
              println!("low");
          }
          else if num ==secret_number {
              println!("correct");
          }
          else if num <= 100 {
              println!("high");
          }
          else {
              println!("out of range {} ",num);
              continue;
          }
      
          //switch case
          match num.cmp(&secret_number) {
              Ordering::Less => println!("small value"),
              Ordering::Equal =>{
                  println!("Guessed correctly");
                  break;
                  },
              Ordering::Greater =>println!("high value")
          }

    }
        
}
