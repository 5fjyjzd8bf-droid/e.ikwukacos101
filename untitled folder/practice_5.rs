use std::io;

fn main()
 {
    let mut input = String::new();

    println!("\n Enter your height (in centimetres);");
    io::stdin().read_line(&mut input).expect("Not a valid String");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        print!("You are of average height");
    }
    else if height > 170.0 && height <= 195.0
    {
        print!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("you are a dwarf");
    }
    else
     {
        panic!("Abnormal height");
      }
}
