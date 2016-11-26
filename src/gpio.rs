pub trait Pin {
    fn high(&mut self);
    fn low(&mut self);
}

pub struct DummyPin;

impl Pin for DummyPin {
    fn high(&mut self) {}
    fn low(&mut self) {}
}

pub trait PinGroup {
    fn write<F: Fn(usize)>(&mut self, data: u8, sleep: &F);
    fn write_u4<F: Fn(usize)>(&mut self, data: u8, sleep: &F);
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

impl<P: Pin, E: Pin> PinGroup for ([P; 8], E) {
    fn write<F: Fn(usize)>(&mut self, data: u8, sleep: &F) {
        for i in 0..8 {
            write_bit!(data & (1 << i) => self.0[i]);
        }
        self.1.high();
        sleep(1);
        self.1.low();
        sleep(100);
    }

    fn write_u4<F: Fn(usize)>(&mut self, data: u8, sleep: &F) {
        self.write(data << 4, sleep);
    }

    fn is_8_bit() -> bool { true }
}

impl<P: Pin, E: Pin> PinGroup for ([P; 4], E) {
    fn write_u4<F: Fn(usize)>(&mut self, data: u8, sleep: &F) {
        self.1.low();
        for i in 0..4 {
            write_bit!(data & (1 << i) => self.0[i]);
        }
        self.1.high();
        sleep(1);
        self.1.low();
        sleep(100);
    }

    fn write<F: Fn(usize)>(&mut self, data: u8, sleep: &F) {

        self.write_u4(data >> 4, sleep);
        self.write_u4(data, sleep);
    }

    fn is_8_bit() -> bool { false }
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, E> PinGroup
    for ((P0, P1, P2, P3, P4, P5, P6, P7), E)
    where P0: Pin,
          P1: Pin,
          P2: Pin,
          P3: Pin,
          P4: Pin,
          P5: Pin,
          P6: Pin,
          P7: Pin,
          E: Pin,
{
    fn write<F: Fn(usize)>(&mut self, data: u8, sleep: &F) {
        write_bit!(data & (1 << 0) => (self.0).0);
        write_bit!(data & (1 << 1) => (self.0).1);
        write_bit!(data & (1 << 2) => (self.0).2);
        write_bit!(data & (1 << 3) => (self.0).3);
        write_bit!(data & (1 << 4) => (self.0).4);
        write_bit!(data & (1 << 5) => (self.0).5);
        write_bit!(data & (1 << 6) => (self.0).6);
        write_bit!(data & (1 << 7) => (self.0).7);
        self.1.high();
        sleep(1);
        self.1.low();
        sleep(100);
    }

    fn write_u4<F: Fn(usize)>(&mut self, data: u8, sleep: &F) {
        self.write(data << 4, sleep);
    }

    fn is_8_bit() -> bool { true }
}

impl<P4, P5, P6, P7, E> PinGroup
    for ((P4, P5, P6, P7), E)
    where P4: Pin,
          P5: Pin,
          P6: Pin,
          P7: Pin,
          E: Pin,
{
    fn write_u4<F: Fn(usize)>(&mut self, data: u8, sleep: &F) {
        write_bit!(data & (1 << 0) => (self.0).0);
        write_bit!(data & (1 << 1) => (self.0).1);
        write_bit!(data & (1 << 2) => (self.0).2);
        write_bit!(data & (1 << 3) => (self.0).3);
        self.1.high();
        sleep(1);
        self.1.low();
        sleep(100);
    }

    fn write<F: Fn(usize)>(&mut self, data: u8, sleep: &F) {
        self.write_u4(data >> 4, sleep);
        self.write_u4(data, sleep);
    }

    fn is_8_bit() -> bool { false }
}
