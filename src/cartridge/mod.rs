pub const NINTENDO_LOGO_LENGTH: usize = 156;

pub struct Header {
    rom_entry_point: u32,
    nintendo_logo: [u8; NINTENDO_LOGO_LENGTH],
    game_title: [u8; 12]
}