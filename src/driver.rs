use commands::{CharacterGrid, Command, Direction, LineCount, TextDirection};
use gpio::{PinGroup, Sleep};
use gpio_traits::pin;

/// Main trait for controlling a HD44780 display.
pub trait Driver {
    /// Clears the display.
    ///
    /// This will sleep for 2ms.
    fn clear_display(&mut self);

    /// Moves the cursor to the start of the memory.
    ///
    /// This will sleep for 2ms.
    fn return_home(&mut self);

    /// Sets what happens when data is written to (or read from) the ddram.
    ///
    /// * `text_direction`:
    ///   * If `LeftToRight`, increment the address pointer after read/write.
    ///   * If `RightToLeft`, decrement the address pointer.
    ///
    /// * `auto_shift_display`: if `true`, writing a character shifts the
    ///   display as well as the address pointer (in the same direction).
    fn set_entry_mode(
        &mut self, text_direction: TextDirection, auto_shift_display: bool
    );

    /// Controls what appears on the LCD.
    ///
    /// * When `display = false`, nothing is shown.
    /// * When `cursor = true`, a line is drawn under the character pointed to
    ///   by the Address Counter.
    /// * When `blinking = true`, the character pointed to by the Address
    ///   Counter will blink.
    fn control_display(&mut self, display: bool, cursor: bool, blinking: bool);

    /// Shifts the display to the given direction.
    ///
    /// Does not change the data memory.
    fn shift_display(&mut self, direction: Direction);

    /// Moves the cursor in the given direction.
    fn shift_cursor(&mut self, direction: Direction);

    /// Sets various parameters for the controller.
    ///
    /// * `lines` sets the number of lines to use (1 or 2).
    /// * `characters` sets the pixel size for a character.
    fn set_function(&mut self, lines: LineCount, characters: CharacterGrid);

    /// Sets the address counter to the given cgram memory address.
    fn set_cgram_address(&mut self, addr: u8);

    /// Sets the address counter to the given dgram memory address.
    fn set_ddram_address(&mut self, addr: u8);

    /// Writes a byte of data to the current address.
    ///
    /// Depending on the controller mode, this may:
    /// * Increment/Decrement the address counter
    /// * Shift the display
    /// * Move the cursor
    fn write_data(&mut self, data: u8);

    /// Write an entire slice of data.
    ///
    /// This will send the bytes one by one.
    fn write_slice(&mut self, data: &[u8]);

    /// Defines a custom character.
    ///
    /// Define a new custom character.
    /// * If the current character grid is 5x8, 8 glyphs can be defined.
    /// * If the current character grid is 5x10, 4 glyphs can be defined.
    ///
    /// `data` must be a slice of bytes, one for each row of the character.
    /// Each byte must only define the lowest 5 bits
    /// (the most-significant 3 bits are ignored).
    fn define_glyph(&mut self, glyph_id: u8, data: &[u8]) {
        // TODO: check data length, compare it to font size
        self.set_cgram_address(glyph_id << 3);
        self.write_slice(data);
    }

    /// Writes a slice of data at the given position.
    ///
    /// `row` must be 0 or 1.
    fn write_at(&mut self, row: u8, col: u8, data: &[u8]) {
        self.set_ddram_address(col + row * 0x40);
        self.write_slice(data);
    }

    /// Moves the cursor to the given position.
    fn set_cursor(&mut self, row: u8, col: u8) {
        self.set_ddram_address(col + row * 0x40);
    }
}

/// GPIO-based implementation of the Driver trait.
///
/// This driver is generic on the actual pins used to communicate with the
/// device.  You will need to implement the `pin::Output`, `PinGroup` and
/// `Sleep` traits for your specific device.
pub struct PinDriver<RS, RW, Data, SleepFn>
where
    RS: pin::Output,
    RW: pin::Output,
    Data: PinGroup,
    SleepFn: Sleep,
{
    rs: RS,
    rw: RW,
    data: Data,
    sleep: SleepFn,
}

impl<RS, RW, Data, SleepFn> PinDriver<RS, RW, Data, SleepFn>
where
    RS: pin::Output,
    RW: pin::Output,
    Data: PinGroup,
    SleepFn: Sleep,
{
    /// Creates a new driver using the given pins.
    ///
    /// * `rw` can be a dummy pin. In that case, be sure to connect it to LOW.
    /// * `data` can be either 4-pins or 8-pins.
    pub fn new(rs: RS, rw: RW, data: Data, sleep: SleepFn) -> Self {
        let mut driver = PinDriver {
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
        self.data.write_u4(data, &mut self.sleep);
    }

    fn write(&mut self, data: u8) {
        self.data.write(data, &mut self.sleep);
    }

    fn initialize_bits(&mut self) {
        // We don't know what mode the module is in.
        // (It could be either in 4-bits or in 8-bits mode)
        // So first, set it to 8-bits mode.

        // If we're in 8 bits mode, this should have no effect twice.
        self.write_u4(0x3);
        self.write_u4(0x3);
        // If we were in 4 bits mode, this were two halves of a single command,
        // designed to move to 8-bits mode.

        // There, we should be in 8-bits mode now.

        if !Data::is_8_bit() {
            // Now, switch back to 4 bits if that's what we want.
            // Going temporarily to 8-bits was the best way to ensure we
            // don't end up half-way in a 8-bits command.
            self.write_u4(0x2);
        }
    }
}

impl<RS, RW, Data, SleepFn> Driver for PinDriver<RS, RW, Data, SleepFn>
where
    RS: pin::Output,
    RW: pin::Output,
    Data: PinGroup,
    SleepFn: Sleep,
{
    fn clear_display(&mut self) {
        self.rs.low();
        self.write(Command::ClearDisplay as u8);
        // This method is slower than most.
        self.sleep.sleep(2000);
    }

    fn return_home(&mut self) {
        self.rs.low();
        self.write(Command::ReturnHome as u8);
        // This method is slower than most.
        self.sleep.sleep(2000);
    }

    fn set_entry_mode(
        &mut self, text_direction: TextDirection, auto_shift_display: bool
    ) {
        let id = text_direction as u8;
        let sh = auto_shift_display as u8;

        self.rs.low();
        self.write(Command::SetEntryMode as u8 | id | sh);
    }

    fn control_display(
        &mut self, display: bool, cursor: bool, blinking: bool
    ) {
        use commands::{show_blinking, show_cursor, show_display};

        let d = show_display(display);
        let c = show_cursor(cursor);
        let b = show_blinking(blinking);
        self.rs.low();
        self.write(Command::ControlDisplay as u8 | d | c | b);
    }

    fn shift_display(&mut self, direction: Direction) {
        let rl = direction as u8;
        self.rs.low();
        self.write(Command::ShiftDisplay as u8 | rl);
    }

    fn shift_cursor(&mut self, direction: Direction) {
        let rl = direction as u8;
        self.rs.low();
        self.write(Command::ShiftCursor as u8 | rl);
    }

    fn set_function(&mut self, lines: LineCount, characters: CharacterGrid) {
        use commands::line_count;

        let dl = line_count(Data::is_8_bit());
        let n = lines as u8;
        let f = characters as u8;

        self.rs.low();
        self.write(Command::SetFunction as u8 | dl | n | f);
    }

    fn set_cgram_address(&mut self, addr: u8) {
        use commands::cgram_mask;

        self.rs.low();
        self.write(Command::SetCgramAddr as u8 | cgram_mask(addr));
    }

    fn set_ddram_address(&mut self, addr: u8) {
        self.rs.low();
        self.write(Command::SetDdramAddr as u8 | addr);
    }

    fn write_data(&mut self, data: u8) {
        self.rs.high();
        self.write(data);
    }

    fn write_slice(&mut self, data: &[u8]) {
        self.rs.high();

        for &byte in data {
            self.write(byte);
        }
    }

    // Let's not expose any read method, it's easier this way.
    //
    // pub fn read_data(&mut self) -> u8 { 0 }
    // pub fn read_bf_addr(&mut self) -> (bool, u8) { (false, 0) }
    //
}
