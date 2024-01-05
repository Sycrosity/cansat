use crate::{
    mpu6050::{MpuSignal, MPU_SIGNAL},
    prelude::*,
};

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

    pub fn clear(&mut self) -> Result<(), TerminalModeError> {
        self.display.clear()
    }

    pub fn quick_clear(&mut self) {
        if let Err(e) = self.set_position(0, 0) {
            warn!("{e:?}");
            self.clear();
        };
        if let Err(e) = self.display.write_str("") {
            warn!("{e:?}");
            self.clear();
        };
    }

    pub fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.display.write_str(s)
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), fmt::Error> {
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

        display.quick_clear();
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
pub async fn display_numerical_data(
    mut display: Display<DisplaySize128x64>,
    //  control: &'static MpuSignal
) {
    loop {
        let mpu_data = MPU_SIGNAL.wait().await;

        // Timer::after_millis(1).await;

        display.quick_clear();

        if let Err(e) = display
        .write_fmt(format_args!(
            "temp: {:.4}c\nacc: (x,y,z)\n{:.1}, {:.1}, {:.1}\ngyro:\n{:.0}, {:.0}, {:.0}\nroll/pitch:\n{:.2}, {:.2}",
            mpu_data.temp,
            mpu_data.acc.x,
            mpu_data.acc.y,
            mpu_data.acc.z,
            mpu_data.gyro.x,
            mpu_data.gyro.y,
            mpu_data.gyro.z,
            mpu_data.roll_pitch.x,
            mpu_data.roll_pitch.y
        )) {
            warn!("{e:?}");
            display.clear();
        };

        // display
        //     .write_fmt(format_args!(
        //         "temp: {:.4}c\nacc: (x,y,z)\n{:.1}, {:.1}, {:.1}\ngyro:\n{:.1},{:.1},{:.1}\nroll/pitch:\n{:.2}, {:.2}",
        //         mpu_data.temp,
        //         mpu_data.acc.x,
        //         mpu_data.acc.y,
        //         mpu_data.acc.z,
        //         mpu_data.gyro.x,
        //         mpu_data.gyro.y,
        //         mpu_data.gyro.z,
        //         mpu_data.roll_pitch.x,
        //         mpu_data.roll_pitch.y
        //     ))
        // .unwrap();
    }
}
