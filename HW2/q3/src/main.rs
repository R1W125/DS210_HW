use std::io;
//u8 holds up to 255
fn main() {
    let mut sum:u32=0; 
    let mut input =String ::new(); 
    io::stdin().read_line(&mut input).expect("Failed to read line"); 
    let input = input.trim(); 
    let number: u8 = input.parse().expect("Not a good number!");
    for i in 1..=number{
        sum += (i as u32).pow(3); //u32 holds 4294967295
    }
    println!("{}", sum);

}
