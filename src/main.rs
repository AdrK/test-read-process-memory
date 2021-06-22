extern crate libc;
extern crate benfred_read_process_memory;
extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian};

use benfred_read_process_memory::*;
use std::convert::TryInto;
use std::env;

fn main() {
    let pid = env::args().nth(1).unwrap().parse::<usize>().unwrap() as Pid;
    let addr = usize::from_str_radix(&env::args().nth(2).unwrap(), 16).unwrap();
    let size = env::args().nth(3).unwrap().parse::<usize>().unwrap();
    let handle: ProcessHandle = pid.try_into().unwrap();
    copy_address(addr, size, &handle)
        .map_err(|e| {
            println!("Error: {:?}", e);
            e
        })
        .map(|bytes| {
            println!("{} bytes at address {:x}:
{}
",
                     size,
                     addr,
                     LittleEndian::read_u32(&bytes))
        })
        .unwrap();
}
