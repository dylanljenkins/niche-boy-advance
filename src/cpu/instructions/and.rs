use crate::cpu::cpu::Emu;
use crate::cpu::register::Register;

pub fn and(emu: &mut Emu, register: &Register) -> u8 {
    emu.registers.accumulator = emu.registers.accumulator & emu.registers.get_value(register);
    1
}

pub fn and_immediate(emu: &mut Emu) -> u8 {
    emu.registers.accumulator = emu.registers.accumulator & emu.read_and_inc();
    2
}