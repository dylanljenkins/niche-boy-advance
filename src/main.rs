mod cpu;
mod util;
mod gpu;

use crate::cpu::cpu::CPU;

use std::fs::File;
use std::io::prelude::*;
use crate::cpu::opcodes::OpCodes;
use std::borrow::Borrow;
use minifb::{Window, WindowOptions, Scale};
use crate::gpu::gpu::GPU;

const T_CLOCK: u32 = 4194304u32;
const M_CLOCK: u32 = T_CLOCK / 4;
const FPS: u16 = 60;

#[derive(Debug)]
#[derive(Default)]
struct Emulator {
    cpu: CPU,
}

impl Emulator {
    fn run(&mut self) {
        // eprintln!("rom[0] = {:x}", &self.rom[0]);
        // eprintln!("cpu = {:#?}", &self.cpu);
        self.cpu.registers.pc = 0x100;
        self.cpu.registers.accumulator = 0x1;
        self.cpu.registers.sp = 0xFFFE;
        self.main_loop();
    }

    fn main_loop(&mut self) {

        let runner = OpCodes {};

        let mut buffer = vec![0u32; 160*144];

        let mut window = Window::new(
            "Test - ESC to exit",
            160,
            144,
            WindowOptions { scale: Scale::X4, ..WindowOptions::default() }
        ).unwrap();

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        loop {
            let opcode = self.cpu.read_and_inc();
            println!("{:#4X?}", opcode);
            let m_cycles = runner.run_op(opcode, &mut self.cpu);

            if m_cycles == 0 {
                // Unknown.
                println!("Unknown opcode: {:#4X?}", opcode);
                break;
            }

            if window.is_open() {
               window.update_with_buffer(&buffer, 160, 144) .unwrap();
            }
            // println!("{}", cycles);

            println!("{:?}", &self.cpu.registers);
            println!("{:?}", &self.cpu.registers.flags);
        }
        // eprintln!("opcode = {:#?}", opcode);
    }
}


fn main() {

    // let rom = File::open("C:\\Users\\Dylan\\Downloads\\Pokemon - Red Version (UE)[!]\\Pokemon Red.gb");

    // Maximum size of GB ROM: http://www.codeslinger.co.uk/pages/projects/gameboy/beginning.html
    // let mut rom = [0u8; 200000];
    let mut boot = File::open("src/Pokemon Red.gb").unwrap();
    // let mut boot = File::open("src/dmg_boot.bin").unwrap();

    let mut emu = Emulator::default();
    boot.read(&mut emu.cpu.memory.buffer).unwrap();

    emu.run();


    // let he = hex::encode(rom);
    // println!("{}", he);

    // println!("{:?}", rom);

    // println!("Hello, world!");
}