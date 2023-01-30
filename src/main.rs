use std::process;
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_secs(3);
    let mut counter = 0;
    loop{
        println!("sleeping for 3  sec ");
        thread::sleep(delay);
        counter += 1;
        if counter > 10 {
            process::exit(0x0100);
        }
    }
}

