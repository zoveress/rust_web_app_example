use std::time::Duration;
use std::thread::sleep;


fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;
        println!("{}", count);
        //println!("Counting: ", count);
        println!("Current number = {}", count);
        sleep(Duration::from_secs(5));

    }
}