use chrono::prelude::*;

fn main() {
    let utc = Utc::now();
    let local_time = Local::now();
    println!("{}", utc);
    println!("{}", local_time);
}
