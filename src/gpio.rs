use gpio_traits::pin;

/// Trait for a sleep function.
pub trait Sleep {
    /// Wait for the given number of microseconds.
    fn sleep(&mut self, us: u32);
}

impl<F> Sleep for F
where
    F: FnMut(u32),
{
    fn sleep(&mut self, us: u32) {
        self(us)
    }
}

/// A group of pins used for data transfer.
///
/// This is usually a group of 8 pins, able to transfer a byte at a time.
/// A 4-pins mode is also possible, where a byte is transfered in two steps.
///
/// This trait is implemented for arrays and tuples of 4 and 8 pins.
///
/// You can also implement this directly, if you use a pin multiplexer for
/// instance.
pub trait PinGroup {
    /// Send a full 8-bit command, using the given sleep function.
    fn write<F: Sleep>(&mut self, data: u8, sleep: &mut F);

    /// Send a 4-bits command.
    fn write_u4<F: Sleep>(&mut self, data: u8, sleep: &mut F);

    /// Should return `true` if this group uses 8 pins.
    fn is_8_bit() -> bool;
}

macro_rules! write_bit {
    ($bit:expr => $pin:expr) => {
        if $bit != 0 {
            $pin.high();
        } else {
            $pin.low();
        }
    }
}

impl<P: pin::Output, E: pin::Output> PinGroup for ([P; 8], E) {
    fn write<F: Sleep>(&mut self, data: u8, sleep: &mut F) {
        for i in 0..8 {
            write_bit!(data & (1 << i) => self.0[i]);
        }
        self.1.high();
        sleep.sleep(1);
        self.1.low();
        sleep.sleep(100);
    }

    fn write_u4<F: Sleep>(&mut self, data: u8, sleep: &mut F) {
        self.write(data << 4, sleep);
    }

    fn is_8_bit() -> bool {
        true
    }
}

impl<P: pin::Output, E: pin::Output> PinGroup for ([P; 4], E) {
    fn write_u4<F: Sleep>(&mut self, data: u8, sleep: &mut F) {
        self.1.low();
        for i in 0..4 {
            write_bit!(data & (1 << i) => self.0[i]);
        }
        self.1.high();
        sleep.sleep(1);
        self.1.low();
        sleep.sleep(100);
    }

    fn write<F: Sleep>(&mut self, data: u8, sleep: &mut F) {
        self.write_u4(data >> 4, sleep);
        self.write_u4(data, sleep);
    }

    fn is_8_bit() -> bool {
        false
    }
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, E> PinGroup
    for ((P0, P1, P2, P3, P4, P5, P6, P7), E)
where
    P0: pin::Output,
    P1: pin::Output,
    P2: pin::Output,
    P3: pin::Output,
    P4: pin::Output,
    P5: pin::Output,
    P6: pin::Output,
    P7: pin::Output,
    E: pin::Output,
{
    fn write<F: Sleep>(&mut self, data: u8, sleep: &mut F) {
        write_bit!(data & (1 << 0) => (self.0).0);
        write_bit!(data & (1 << 1) => (self.0).1);
        write_bit!(data & (1 << 2) => (self.0).2);
        write_bit!(data & (1 << 3) => (self.0).3);
        write_bit!(data & (1 << 4) => (self.0).4);
        write_bit!(data & (1 << 5) => (self.0).5);
        write_bit!(data & (1 << 6) => (self.0).6);
        write_bit!(data & (1 << 7) => (self.0).7);
        self.1.high();
        sleep.sleep(1);
        self.1.low();
        sleep.sleep(100);
    }

    fn write_u4<F: Sleep>(&mut self, data: u8, sleep: &mut F) {
        self.write(data << 4, sleep);
    }

    fn is_8_bit() -> bool {
        true
    }
}

impl<P4, P5, P6, P7, E> PinGroup for ((P4, P5, P6, P7), E)
where
    P4: pin::Output,
    P5: pin::Output,
    P6: pin::Output,
    P7: pin::Output,
    E: pin::Output,
{
    fn write_u4<F: Sleep>(&mut self, data: u8, sleep: &mut F) {
        write_bit!(data & (1 << 0) => (self.0).0);
        write_bit!(data & (1 << 1) => (self.0).1);
        write_bit!(data & (1 << 2) => (self.0).2);
        write_bit!(data & (1 << 3) => (self.0).3);
        self.1.high();
        sleep.sleep(1);
        self.1.low();
        sleep.sleep(100);
    }

    fn write<F: Sleep>(&mut self, data: u8, sleep: &mut F) {
        self.write_u4(data >> 4, sleep);
        self.write_u4(data, sleep);
    }

    fn is_8_bit() -> bool {
        false
    }
}
