use std::time::SystemTime;

fn main() {
    println!("Getting the time.");

    //let now = SystemTime::now().elapsed().unwrap().as_secs();
    if let Ok(t) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        println!("POSIX time: {}", t.as_secs());
    } else {
        panic!("SystemTime::now() failed");
    }
}
