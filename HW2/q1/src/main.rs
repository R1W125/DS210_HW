use std::time::SystemTime;

fn fib(k:u32) -> u128{
    match k{
        0 => 0 as u128,
        1 => 1 as u128, 
        _ => fib(k-1)+fib(k-2) as u128
    }
}

fn main() {
    
    for i in 0..=49{
        let before = SystemTime::now();
        println!("k={}, F({})={}", i, i, fib(i));
        let after = SystemTime::now();
        let difference = after.duration_since(before);
        let difference = difference.expect("Did the clock go back?");
        println!("Time it took: {:?}", difference)
    }

}