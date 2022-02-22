extern crate wwserial;

use wwserial::WwSerial;

fn main() {
    let s = WwSerial::new("COM3".to_string(), 115200, 20, 20, true);

    let x: Vec<u8> = vec![0xff, 0x10, 0x00, 0xf1];
    println!("Sending {:?}", x);
    let bytes = s.write(&x);
    println!("Sent {}/4", bytes);

    let mut r: Vec<u8> = Vec::with_capacity(100);
    s.read(&mut r);
    println!("Received {:?}", r);

    let x: Vec<u8> = vec![0xff, 0xf0, 0x00, 0x11];
    println!("Sending {:?}", x);
    let bytes = s.write(&x);
    println!("Sent {}/4", bytes);

    let mut r: Vec<u8> = Vec::with_capacity(100);
    s.read(&mut r);
    println!("Received {:?}", r);

    let x: Vec<u8> = vec![0xff, 0x03, 0x00, 0xfe];
    println!("Sending {:?}", x);
    let bytes = s.write(&x);
    println!("Sent {}/4", bytes);

    let mut r: Vec<u8> = Vec::with_capacity(100);
    s.read(&mut r);
    println!("Received {:?}", r);

    let x: Vec<u8> = vec![0xff; 130];
    println!("Sending {:?}", x);
    let bytes = s.write(&x);
    println!("Sent {}/130", bytes);

    println!("Infinite looping, ctrl-c to quit");
    loop {
        let mut r: Vec<u8> = Vec::with_capacity(500);
        let bytes = s.read(&mut r);
        if bytes > 0 {
            println!("(ctrl-c to quit) Received ({}) {:?}", bytes, r);

            {
                let x: Vec<u8> = vec![0xff; 130];
                println!("Sending {:?}", x);
                let bytes = s.write(&x);
                println!("Sent {}/130", bytes);
            }
        }
    }
}
