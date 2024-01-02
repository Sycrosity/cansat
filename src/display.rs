use cansat::prelude::*;

// use alloc::format;
use core::fmt::{Write, self};
use esp32_hal::{i2c::I2C, peripherals::I2C0};
use ssd1306::{
    mode::{TerminalDisplaySize, TerminalMode},
    prelude::*,
    I2CDisplayInterface, Ssd1306,
};

type DisplayInternals<SIZE> = Ssd1306<I2CInterface<I2C<'static, I2C0>>, SIZE, TerminalMode>;

pub struct Display<SIZE> {
    display: DisplayInternals<SIZE>,
}

impl<SIZE: DisplaySize + TerminalDisplaySize> Display<SIZE> {
    pub fn new(i2c: I2C<'static, I2C0>, display_size: SIZE) -> Self {
        let interface = I2CDisplayInterface::new(i2c);

        let mut display =
            Ssd1306::new(interface, display_size, DisplayRotation::Rotate0).into_terminal_mode();

        display.init().unwrap();
        display.clear().unwrap();

        Self { display }
    }

    pub fn clear(&mut self) {

        self.display.set_position(0, 0).unwrap();
        self.display.write_str("").unwrap();

    }

    pub fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {

        self.clear();
        self.display.write_str(s)

    }

    pub fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), fmt::Error> {

        self.clear();
        self.display.write_fmt(args)

    }
}

#[task]
pub async fn screen_counter(mut display: Display<DisplaySize128x64>) {
    let mut counter: u16 = 0;

    loop {

        display.write_fmt(format_args!("{}", counter)).unwrap();

        counter = match counter.checked_add(1) {
            Some(new) => new,
            None => {
                display.display.clear().unwrap();
                1
            }
        };
    }
}
