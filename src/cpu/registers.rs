use bitflags::bitflags;

pub const BASE_ADDRESS: u32 = 0x0000_0000;
pub const GENERAL_PURPOSE_REGISTERS_COUNT: usize = 14;
pub const FIQ_REGISTERS_COUNT: usize = 7;
pub const SUPERVISOR_REGISTERS_COUNT: usize = 2;
pub const ABORT_REGISTERS_COUNT: usize = 2;
pub const IRQ_REGISTERS_COUNT: usize = 2;
pub const UNDEFINED_REGISTERS_COUNT: usize = 2;


const fn make_n_shifted_bits(n: u32, shift: u32) -> u32 {
    if n == 0 {
        return 1 << shift;
    }

    return ((1 << n) - 1) << shift; 
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    System,
    Fiq,
    Supervisor,
    Abort,
    Irq,
    Undefined,
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct StatusFlags: u32 {
        const SIGNED          = make_n_shifted_bits(1, 31);
        const ZERO            = make_n_shifted_bits(1, 30);
        const CARRY           = make_n_shifted_bits(1, 29);
        const OVERFLOW        = make_n_shifted_bits(1, 28);
        const STICKY_OVERFLOW = make_n_shifted_bits(1, 27);
        const RESERVED_26_25  = make_n_shifted_bits(2, 25);
        const JAZELLE_MODE    = make_n_shifted_bits(1, 24);
        const RESERVED_23_10  = make_n_shifted_bits(13, 10);
        const BIG_ENDIAN      = make_n_shifted_bits(1, 9);
        const ABORT_DISABLE   = make_n_shifted_bits(1, 8);
        const IRQ_DISABLE     = make_n_shifted_bits(1, 7);
        const FIQ_DISABLE     = make_n_shifted_bits(1, 6);
        const STATE           = make_n_shifted_bits(1, 5);
        const M4              = make_n_shifted_bits(1, 4);
        const M3              = make_n_shifted_bits(1, 3);
        const M2              = make_n_shifted_bits(1, 2);
        const M1              = make_n_shifted_bits(1, 1);
        const M0              = make_n_shifted_bits(1, 0);
    }
}

pub struct StatusRegister {
    pub cpsr: StatusFlags,
    
    pub spsr_fiq: StatusFlags,
    pub spsr_svc: StatusFlags,
    pub spsr_abt: StatusFlags,
    pub spsr_irq: StatusFlags,
    pub spsr_und: StatusFlags,
}

pub struct Registers {
    pub register: [u32; GENERAL_PURPOSE_REGISTERS_COUNT],
    pub program_counter: u32,

    pub fiq_register: [u32; FIQ_REGISTERS_COUNT],
    pub svc_register: [u32; SUPERVISOR_REGISTERS_COUNT],
    pub abort_register: [u32; ABORT_REGISTERS_COUNT],
    pub irq_register: [u32; IRQ_REGISTERS_COUNT],
    pub und_register: [u32; UNDEFINED_REGISTERS_COUNT],
}

impl StatusRegister {
    pub fn new() -> Self {
        return StatusRegister {
            cpsr: StatusFlags::empty(),

            spsr_fiq: StatusFlags::empty(),
            spsr_svc: StatusFlags::empty(),
            spsr_abt: StatusFlags::empty(),
            spsr_irq: StatusFlags::empty(),
            spsr_und: StatusFlags::empty(),
        };
    }
}

impl Registers {
    pub fn new() -> Self {
        return Registers {
            register: [0; GENERAL_PURPOSE_REGISTERS_COUNT],
            program_counter: BASE_ADDRESS,

            fiq_register: [0; FIQ_REGISTERS_COUNT],
            svc_register: [0; SUPERVISOR_REGISTERS_COUNT],
            abort_register: [0; ABORT_REGISTERS_COUNT],
            irq_register: [0; IRQ_REGISTERS_COUNT],
            und_register: [0; UNDEFINED_REGISTERS_COUNT],
        };
    }

    pub fn set_register(&mut self, index: usize, value: u32) {
        self.register[index] = value;
    }

    pub fn get_register(&self, index: usize) -> u32 {
        return self.register[index];
    }

    pub fn set_register_by_mode(&mut self, mode: Mode, index: usize, value: u32) {
        match mode {
            Mode::System => {
                self.set_register(index, value);
            },
            Mode::Fiq => {
                self.fiq_register[index - 8] = value;
            },
            Mode::Supervisor => {
                self.svc_register[index - 13] = value;
            },
            Mode::Abort => {
                self.abort_register[index - 13] = value;
            },
            Mode::Irq => {
                self.irq_register[index - 13] = value;
            },
            Mode::Undefined => {
                self.und_register[index - 13] = value;
            }
        }
    }

    pub fn get_register_by_mode(&self, mode: Mode, index: usize) -> u32 {
        match mode {
            Mode::System => {
                return self.get_register(index);
            },
            Mode::Fiq => {
                return self.fiq_register[index - 8];
            },
            Mode::Supervisor => {
                return self.svc_register[index - 13];
            },
            Mode::Abort => {
                return self.abort_register[index - 13];
            },
            Mode::Irq => {
                return self.irq_register[index - 13];
            },
            Mode::Undefined => {
                return self.und_register[index - 13];
            }
        }
    }
}