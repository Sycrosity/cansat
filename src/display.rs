use crate::prelude::*;

// use alloc::format;
use core::fmt::{self, Write};
use esp32_hal::{i2c::I2C, peripherals::I2C0};
use ssd1306::{
    mode::{TerminalDisplaySize, TerminalMode, TerminalModeError},
    prelude::*,
    I2CDisplayInterface, Ssd1306,
};

type DisplayInternals<SIZE> = Ssd1306<I2CInterface<I2CShared>, SIZE, TerminalMode>;

pub struct Display<SIZE> {
    display: DisplayInternals<SIZE>,
}

impl<SIZE: DisplaySize + TerminalDisplaySize> Display<SIZE> {
    pub fn new(i2c: I2CShared, display_size: SIZE) -> Self {
        let interface = I2CDisplayInterface::new(i2c);

        let mut display =
            Ssd1306::new(interface, display_size, DisplayRotation::Rotate0).into_terminal_mode();

        display.init().unwrap();

        display.clear().unwrap();

        Self { display }
    }

    pub fn clear(&mut self) {
        self.set_position(0, 0);
        self.display.write_str("");
    }

    pub fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.clear();
        self.display.write_str(s)
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), fmt::Error> {
        self.clear();
        self.display.write_fmt(args)
    }

    pub fn set_position(&mut self, column: u8, row: u8) -> Result<(), TerminalModeError> {
        self.display.set_position(column, row)
    }
}

#[task]
pub async fn screen_counter(mut display: Display<DisplaySize128x64>) {
    let mut counter: u16 = 0;

    loop {
        Timer::after_millis(1).await;

        display.write_fmt(format_args!("{}", counter)).unwrap();

        counter = match counter.checked_add(1) {
            Some(next) => next,
            None => {
                display.clear();
                1
            }
        };
    }
}

#[task]
pub async fn display_numerical_data(mut display: Display<DisplaySize128x64>) {
    let mut counter: u16 = 0;

    loop {
        Timer::after_millis(1).await;

        display.write_fmt(format_args!("{}", counter)).unwrap();

        counter = match counter.checked_add(1) {
            Some(next) => next,
            None => {
                display.clear();
                1
            }
        };
    }
}
