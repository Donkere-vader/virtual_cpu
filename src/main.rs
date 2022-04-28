mod cpu;
mod load;
mod types;
mod utils;

use cpu::{ Cpu };
use load::{ loads };
use std::fs;
use std::io;

fn main() {
    let machinecode_file_contents = fs::read_to_string("machinecode").expect("Error reading file");
    let mem = loads(machinecode_file_contents);

    let mut cpu = Cpu::new(mem);

    while cpu.counter != u16::MAX {
        cpu.tick();
        println!("CPU VALUES:");
        println!("counter: {}", cpu.counter);
        println!("registries: {:?}", cpu.registries);
        println!("Active registry: {}", cpu.active_res);
        println!("mem[5]: {:?}", cpu.mem[5]);
        let mut _buffer = String::new();
        io::stdin().read_line(&mut _buffer).expect("Failed to read from STDIN");
    }
}
