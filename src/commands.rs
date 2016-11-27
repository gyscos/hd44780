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

/// Specifies where to move the pointer after writing/reading data.
#[repr(u8)]
pub enum TextDirection {
    /// Decrement the pointer after reading/writing.
    ///
    /// This means the next character will be to the left.
    RightToLeft = 0,
    /// Increment the pointer after reading/writing.
    ///
    /// This means the next character will be to the right.
    LeftToRight = 1 << 1,
}

impl TextDirection {
    pub fn from_u8(data: u8) -> Self {
        if (data & TextDirection::LeftToRight as u8) != 0 {
            TextDirection::LeftToRight
        } else {
            TextDirection::RightToLeft
        }
    }
}

#[repr(u8)]
pub enum Direction {
    Left = 0,
    Right = 1 << 2,
}

impl Direction {
    pub fn from_u8(data: u8) -> Self {
        if (data & Direction::Right as u8) != 0 {
            Direction::Right
        } else {
            Direction::Left
        }
    }
}

#[repr(u8)]
pub enum LineCount {
    One = 0,
    Two = 1 << 3,
}

impl LineCount {
    pub fn from_u8(data: u8) -> Self {
        if (data & LineCount::Two as u8) != 0 {
            LineCount::Two
        } else {
            LineCount::One
        }
    }
}

#[repr(u8)]
pub enum CharacterGrid {
    C5x8 = 0,
    C5x11 = 1 << 2,
}

impl CharacterGrid {
    pub fn from_u8(data: u8) -> Self {
        if (data & CharacterGrid::C5x11 as u8) != 0 {
            CharacterGrid::C5x11
        } else {
            CharacterGrid::C5x8
        }
    }
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
