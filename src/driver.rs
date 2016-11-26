use gpio::{Pin, PinGroup};

pub struct Driver<RS, RW, Data, Sleep>
    where RS: Pin,
          RW: Pin,
          Data: PinGroup,
          Sleep: Fn(usize)
{
    rs: RS,
    rw: RW,
    data: Data,
    sleep: Sleep,
}

#[repr(u8)]
pub enum TextDirection {
    RightToLeft = 0,
    LeftToRight = 1,
}

#[repr(u8)]
pub enum Direction {
    Left = 0,
    Right = 1,
}

#[repr(u8)]
pub enum LineCount {
    One = 0,
    Two = 1,
}

#[repr(u8)]
pub enum CharacterGrid {
    C5x8 = 0,
    C5x11 = 1,
}

impl<RS, RW, Data, Sleep> Driver<RS, RW, Data, Sleep>
    where RS: Pin,
          RW: Pin,
          Data: PinGroup,
          Sleep: Fn(usize)
{
    /// Creates a new driver using the given pins.
    ///
    /// * `rw` can be a dummy pin. In that case, be sure to connect it to LOW.
    /// * `data` can be either 4-pins or 8-pins.
    pub fn new(rs: RS, rw: RW, data: Data, sleep: Sleep) -> Self {

        let mut driver = Driver {
            rs: rs,
            rw: rw,
            data: data,
            sleep: sleep,
        };

        // Do some initialization
        driver.initialize_bits();
        driver.rw.low();

        driver
    }

    fn write_u4(&mut self, data: u8) {
        self.data.write_u4(data, &self.sleep);
    }

    fn write(&mut self, data: u8) {
        self.data.write(data, &self.sleep);
    }

    fn initialize_bits(&mut self) {
        // We don't know what mode the module is in.
        // So first, set it to 8-bits mode.

        // If we're in 8 bits mode, this should have no effect twice.
        self.write_u4(0x3);
        self.write_u4(0x3);
        // If we were in 4 bits mode, this were two halves of a single command,
        // designed to move to 8-bits mode.

        // There, we should be in 8-bits mode.

        if !Data::is_8_bit() {
            // Now, switch back to 4 bits if that's what we want.
            // Going temporarily to 8-bits was the best way to ensure we
            // don't end up half-way in a 8-bits command.
            self.write_u4(0x2);
        }
    }

    pub fn clear_display(&mut self) {
        self.rs.low();
        self.write(0b00000001);
        // This method is slower than most.
        (self.sleep)(2000);
    }

    pub fn return_home(&mut self) {
        self.rs.low();
        self.write(0b00000010);
        // This method is slower than most.
        (self.sleep)(2000);
    }

    /// Sets what happens when data is written or read from the ddram.
    ///
    /// `text_direction`: if `LeftToRight`, increment the address pointer after
    /// read/write.  if `RightToLeft`, decrement the address pointer.
    ///
    /// `auto_shift_display`: if `true`, writing a character shifts the display
    /// as well as the address pointer (in the same direction).
    pub fn set_entry_mode(&mut self, text_direction: TextDirection,
                          auto_shift_display: bool) {
        let id = (text_direction as u8) << 1;
        let sh = auto_shift_display as u8;

        self.rs.low();
        self.write(0b00000100 | id | sh);
    }

    /// Controls what appears on the LCD.
    ///
    /// * When `display = false`, nothing is shown.
    /// * When `cursor = true`, a line is drawn under the character pointed to
    ///   by the Address Counter.
    /// * When `blinking = true`, the character pointed to by the Address
    ///   Counter will blink.
    pub fn control_display(&mut self, display: bool, cursor: bool,
                           blinking: bool) {
        let d = (display as u8) << 2;
        let c = (cursor as u8) << 1;
        let b = blinking as u8;
        self.rs.low();
        self.write(0b00001000 | d | c | b);
    }

    pub fn shift_display(&mut self, direction: Direction) {
        let rl = (direction as u8) << 2;
        self.rs.low();
        self.write(0b00011000 | rl);
    }

    pub fn shift_cursor(&mut self, direction: Direction) {
        let rl = (direction as u8) << 2;
        self.rs.low();
        self.write(0b00010000 | rl);
    }

    pub fn set_function(&mut self, lines: LineCount,
                        characters: CharacterGrid) {
        let dl = (Data::is_8_bit() as u8) << 4;
        let n = (lines as u8) << 3;
        let f = (characters as u8) << 2;

        self.rs.low();
        self.write(0b00100000 | dl | n | f);
    }

    pub fn set_cgram_address(&mut self, addr: u8) {
        self.rs.low();
        self.write(0b01000000 | (addr & 0b00111111));
    }

    pub fn set_ddram_address(&mut self, addr: u8) {
        self.rs.low();
        self.write(0b10000000 | addr);
    }

    pub fn write_data(&mut self, data: u8) {
        self.rs.high();
        self.write(data);
    }


    // Let's not expose any read method, it's easier this way.
    //
    // pub fn read_data(&mut self) -> u8 { 0 }
    // pub fn read_bf_addr(&mut self) -> (bool, u8) { (false, 0) }
    //
}
