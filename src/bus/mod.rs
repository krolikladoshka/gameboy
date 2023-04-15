pub struct MemoryRange {
    start: u32,
    end: u32,
}

impl MemoryRange {
    pub const fn new(start: u32, end: u32) -> Self {
        return MemoryRange {
            start: start,
            end: end,
        };
    }

    pub fn contains(&self, index: u32) -> bool {
        return self.start <= index && index <= self.end; 
    }
    
    pub const fn length(&self) -> u32 {
        return self.end - self.start;
    }
}

pub const BIOS_ROM: MemoryRange = MemoryRange::new(0x00000000, 0x00003FFF);
pub const NOT_USED_1: MemoryRange = MemoryRange::new(0x00004000, 0x01FFFFFF);
pub const BOARD_WRAM: MemoryRange = MemoryRange::new(0x02000000, 0x0203FFFF);
pub const NOT_USED_2: MemoryRange = MemoryRange::new(0x02040000, 0x02FFFFFF);
pub const CHIP_WRAM_STACK: MemoryRange = MemoryRange::new(0x03007F00, 0x03007FFF);
pub const CHIP_WRAM: MemoryRange = MemoryRange::new(0x03000000, 0x03007FFF);
pub const NOT_USED_3: MemoryRange = MemoryRange::new(0x03008000, 0x03FFFFFF);
pub const IO_REGISTERS: MemoryRange = MemoryRange::new(0x04000000, 0x040003FE);
pub const NOT_USED_4: MemoryRange = MemoryRange::new(0x04000400, 0x04FFFFFF);
