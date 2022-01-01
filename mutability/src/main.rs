/* 

fn main() {
    let x =5;
    println!("The value of x is{}",x);
  }
    x=6;
    println!("The value of x is{}",x);
 }

Generates an error as x cannot be assigned as a variable twice */


fn main() {
    let mut x = 10;                        // mut - mutable
    println!("The value of x is{}",x);
    x= 20;
    println!("The value of x is{}",x);
}




