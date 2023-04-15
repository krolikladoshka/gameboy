pub trait ReadU32 {
    fn read_u32(&mut self, address: u32) -> u32;
}

pub trait WriteU32 {
    fn write_u32(&mut self, address: u32, value: u32);
}

pub trait AccessibleMemoryLocation: ReadU32 + WriteU32 {}

