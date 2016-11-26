#[repr(u8)]
pub enum Command {
    ClearDisplay = 1,
    ReturnHome = 1 << 1,
    SetEntryMode = 1 << 2,
    ControlDisplay = 1 << 3,
    ShiftDisplay = 0b10 << 3,
    ShiftCursor = 0b11 << 3,
    SetFunction = 1 << 5,
    SetCgramAddr = 1 << 6,
    SetDdramAddr = 1 << 7,
}

#[repr(u8)]
pub enum TextDirection {
    RightToLeft = 0,
    LeftToRight = 1 << 1,
}

#[repr(u8)]
pub enum Direction {
    Left = 0,
    Right = 1 << 2,
}

#[repr(u8)]
pub enum LineCount {
    One = 0,
    Two = 1 << 3,
}

#[repr(u8)]
pub enum CharacterGrid {
    C5x8 = 0,
    C5x11 = 1 << 2,
}

pub fn show_display(display: bool) -> u8 {
    (display as u8) << 2
}

pub fn show_cursor(cursor: bool) -> u8 {
    (cursor as u8) << 1
}

pub fn show_blinking(blinking: bool) -> u8 {
    blinking as u8
}

pub fn line_count(is_8_bit: bool) -> u8 {
    (is_8_bit as u8) << 4
}

pub fn cgram_mask(addr: u8) -> u8 {
    addr & 0b00111111
}
