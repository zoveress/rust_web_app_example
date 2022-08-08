use std::time::Duration;
use std::thread::sleep;


fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;
        println!("Counting: ", count);
        sleep(Duration::from_millis(5));

    }
}