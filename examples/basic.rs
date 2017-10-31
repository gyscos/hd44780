extern crate gpio_traits;
extern crate lcd_hd44780;

use gpio_traits::pin;
use lcd_hd44780::Driver;

struct StdoutPin {
    name: String,
}

impl StdoutPin {
    fn new(label: &str) -> Self {
        StdoutPin {
            name: label.to_string(),
        }
    }
}

impl pin::Output for StdoutPin {
    fn high(&mut self) {
        println!("HIGH: {}", self.name);
    }

    fn low(&mut self) {
        println!("LOW:  {}", self.name);
    }
}

fn main() {
    let rs = StdoutPin::new("rs");
    let rw = StdoutPin::new("rw");

    let e = StdoutPin::new("e");

    let d1 = StdoutPin::new("d1");
    let d2 = StdoutPin::new("d2");
    let d3 = StdoutPin::new("d3");
    let d4 = StdoutPin::new("d4");
    let d5 = StdoutPin::new("d5");
    let d6 = StdoutPin::new("d6");
    let d7 = StdoutPin::new("d7");
    let d8 = StdoutPin::new("d8");

    let sleep =
        |us| std::thread::sleep(std::time::Duration::new(0, 1000 * us));

    let mut driver = lcd_hd44780::PinDriver::new(
        rs,
        rw,
        ([d1, d2, d3, d4, d5, d6, d7, d8], e),
        sleep,
    );

    driver.write_at(0, 0, b"Example!");
}
