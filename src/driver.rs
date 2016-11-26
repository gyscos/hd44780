use commands::{Command, TextDirection, Direction, LineCount, CharacterGrid};
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
        self.write(Command::ClearDisplay as u8);
        // This method is slower than most.
        (self.sleep)(2000);
    }

    pub fn return_home(&mut self) {
        self.rs.low();
        self.write(Command::ReturnHome as u8);
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
        let id = text_direction as u8;
        let sh = auto_shift_display as u8;

        self.rs.low();
        self.write(Command::SetEntryMode as u8 | id | sh);
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
        use commands::{show_display, show_blinking, show_cursor};

        let d = show_display(display);
        let c = show_cursor(cursor);
        let b = show_blinking(blinking);
        self.rs.low();
        self.write(Command::ControlDisplay as u8 | d | c | b);
    }

    pub fn shift_display(&mut self, direction: Direction) {
        let rl = direction as u8;
        self.rs.low();
        self.write(Command::ShiftDisplay as u8 | rl);
    }

    pub fn shift_cursor(&mut self, direction: Direction) {
        let rl = direction as u8;
        self.rs.low();
        self.write(Command::ShiftCursor as u8 | rl);
    }

    pub fn set_function(&mut self, lines: LineCount,
                        characters: CharacterGrid) {
        use commands::line_count;

        let dl = line_count(Data::is_8_bit());
        let n = lines as u8;
        let f = characters as u8;

        self.rs.low();
        self.write(Command::SetFunction as u8 | dl | n | f);
    }

    pub fn set_cgram_address(&mut self, addr: u8) {
        use commands::cgram_mask;

        self.rs.low();
        self.write(Command::SetCgramAddr as u8 | cgram_mask(addr));
    }

    pub fn set_ddram_address(&mut self, addr: u8) {
        self.rs.low();
        self.write(Command::SetDdramAddr as u8 | addr);
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
