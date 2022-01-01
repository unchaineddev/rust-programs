//Find MSB of Integer


    use std::io;

fn main() {
    let mut num:i32 = 0;
    let mut cnt:u32 = 0;
    let mut input = String::new();

    println!("Enter Number:");
    io::stdin().read_line(&mut input)
        .expect("Not a valid String");
    num = input.trim().parse()
        .expect("Not a valid NUmber");

    println!("Binary Number: {:#02b}",num);

    while num>0
    {
        cnt=cnt+1;
        num= num >> 1;
    }

    println!("MSB POSITION is {}",cnt-1);
    }
