// Guessing Game in Rust

   use std::io;                         // Standard I/O Library
      
   fn main() {
       println!("Input Your Guess");
       
   
       let mut guess = String::new();      // let creates a mutable variable
  
      io::stdin()                     
          .read_line(&mut guess)          // &mut to make input mutable
          .expect("Can't read Line");
  
      println!("Your Guess was {}",guess);
  }
  

