use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sec = Duration::from_millis(100);
    loop {
        println!("Hello, rs-builder");
        sleep(sec);
    }
}
