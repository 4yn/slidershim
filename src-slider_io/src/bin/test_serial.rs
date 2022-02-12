extern crate slider_io;

use serialport::available_ports;
use std::io;

fn main() {
    let res = available_ports();
    println!("{:?}", res);
    let mut input = String::new();
    let string = io::stdin().read_line(&mut input).unwrap();
}
