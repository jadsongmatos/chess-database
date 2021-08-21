use std::fs::OpenOptions;
use std::io::prelude::*;
use std::char;

fn main() {
    //let map: [u8; 4] = [1, 2, 3, 4];
    let c:char = 32 as char; 
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("database")
        .unwrap();
    write!(&mut file, "{}",c);
}
