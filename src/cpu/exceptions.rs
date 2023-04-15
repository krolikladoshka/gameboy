use super::registers::{Mode, StatusFlags};

pub const RESET_OFFSET: u32 = 0x00;
pub const UNDEFINED_INSTRUCTION_OFFSET: u32 = 0x04;
pub const SOFTWARE_INTERRUPT_OFFSET: u32 = 0x08;
pub const PREFETCH_ABORT_OFFSET: u32 = 0x0C;
pub const DATA_ABORT_OFFSET: u32 = 0x10;
pub const ADDRESS_EXCEEDS_OFFSET: u32 = 0x14;
pub const NORMAL_INTERRUPT_OFFSET: u32 = 0x18;
pub const FAST_INTERRUPT_OFFSET: u32 = 0x1C;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExceptionType {
    Reset,
    UndefinedInstruction,
    SoftwareInterrupt,
    PrefetchAbort,
    DataAbort,
    AddressExceeds26bit,
    NormalInterrupt,
    FastInterrupt,
}

pub struct Exception {
    pub exception_type: ExceptionType,
    pub address_offset: u32,
    pub priority: usize,
    pub entry_mode: Mode,
    pub interrupt_flags: StatusFlags,
}

impl Exception {
    pub const fn new(
        exception_type: ExceptionType,
        address_offset:u32, priority: usize,
        entry_mode: Mode, interrupt_flags: StatusFlags
    ) -> Self {
        return Exception {
            exception_type: exception_type,
            address_offset: address_offset,
            priority: priority,
            entry_mode: entry_mode,
            interrupt_flags: interrupt_flags,
        };
    }
}

pub const RESET: Exception = Exception::new(
    ExceptionType::Reset,
    RESET_OFFSET,
    1,
    Mode::Supervisor,
    StatusFlags::from_bits_truncate(StatusFlags::IRQ_DISABLE.bits() | StatusFlags::FIQ_DISABLE.bits()),
);

pub const UNDEFINED_INSTRUCTION: Exception = Exception::new(
    ExceptionType::UndefinedInstruction,
    UNDEFINED_INSTRUCTION_OFFSET,
    7,
    Mode::Undefined,
    StatusFlags::IRQ_DISABLE,
);

pub const SOFTWARE_INTERRUPT: Exception = Exception::new(
    ExceptionType::SoftwareInterrupt,
    SOFTWARE_INTERRUPT_OFFSET,
    6,
    Mode::Supervisor,
    StatusFlags::IRQ_DISABLE,
);

pub const PREFETCH_ABORT: Exception = Exception::new(
    ExceptionType::PrefetchAbort,
    PREFETCH_ABORT_OFFSET,
    5,
    Mode::Abort,
    StatusFlags::IRQ_DISABLE,
);

pub const DATA_ABORT: Exception = Exception::new(
    ExceptionType::DataAbort,
    DATA_ABORT_OFFSET,
    2,
    Mode::Abort,
    StatusFlags::IRQ_DISABLE,
);

pub const ADDRESS_EXCEEDS_26_BIT: Exception = Exception::new(
    ExceptionType::AddressExceeds26bit,
    ADDRESS_EXCEEDS_OFFSET,
    2,
    Mode::Supervisor,
    StatusFlags::IRQ_DISABLE,
);

pub const NORMAL_INTERRUPT: Exception = Exception::new(
    ExceptionType::NormalInterrupt,
    NORMAL_INTERRUPT_OFFSET,
    4,
    Mode::Irq,
    StatusFlags::IRQ_DISABLE,
);

pub const FAST_INTERRUPT: Exception = Exception::new(
    ExceptionType::FastInterrupt,
    FAST_INTERRUPT_OFFSET,
    3,
    Mode::Fiq,
    StatusFlags::from_bits_truncate(StatusFlags::IRQ_DISABLE.bits() | StatusFlags::FIQ_DISABLE.bits()),
);
