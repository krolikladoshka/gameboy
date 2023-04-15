use super::registers::{StatusRegister, Registers};

pub const CPU_CYCLES_PER_SECOND: u64 = 1 << 24;

pub enum State {
    ARM,
    THUMB,
}


pub struct Cpu {
    state: State,

    status_register: StatusRegister,
    registers: Registers,
}



impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            state: State::ARM,
            status_register: StatusRegister::new(),
            registers: Registers::new(),
        };

        cpu.reset();

        return cpu;
    }

    pub fn reset(&mut self) {

    }
}
