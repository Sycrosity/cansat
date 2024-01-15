use crate::{mpu6050::MPU_SIGNAL, prelude::*};

use core::fmt::{self, Write};

use ssd1306::{
    mode::{TerminalDisplaySize, TerminalMode, TerminalModeError},
    prelude::*,
    I2CDisplayInterface, Ssd1306,
};

type DisplayInternals<SIZE> = Ssd1306<I2CInterface<SharedI2C>, SIZE, TerminalMode>;

type Result<T> = core::result::Result<T, Error<DisplayError>>;

#[derive(Clone, Copy, ErrorCategory)]
#[error_category(links(DisplayError))]
#[repr(u8)]
pub enum DisplayError {
    /// Display initialisation failed
    InitFailed,
    /// The Display was used before initialisation
    Uninitialised,
    /// An error occured while attempting to write to the screen
    WriteError,
    /// A write location was specified outside of the screen
    OutOfBounds,
    /// An error with the underlying interface of the display
    InterfaceError,
    /// An error while clearing the display's screen
    ClearError,
    /// An error while formatting (from [core::fmt::Error])
    FormatError,
}

impl From<TerminalModeError> for DisplayError {
    fn from(value: TerminalModeError) -> Self {
        match value {
            TerminalModeError::InterfaceError(_) => Self::InterfaceError,
            TerminalModeError::Uninitialized => Self::Uninitialised,
            TerminalModeError::OutOfBounds => Self::OutOfBounds,
        }
    }
}

impl From<core::fmt::Error> for DisplayError {
    fn from(_value: core::fmt::Error) -> Self {
        Self::FormatError
    }
}

pub struct Display<SIZE> {
    display: DisplayInternals<SIZE>,
}

impl<SIZE: DisplaySize + TerminalDisplaySize> Display<SIZE> {
    pub async fn new(i2c: SharedI2C, display_size: SIZE) -> Result<Self>
// where I: Write
    {
        let interface = I2CDisplayInterface::new(i2c);

        let display =
            Ssd1306::new(interface, display_size, DisplayRotation::Rotate0).into_terminal_mode();

        let mut display = Self { display };

        display.init()?;

        display.clear().print_error();

        Ok(display)
    }

    fn init(&mut self) -> Result<()> {
        self.display.init().map_err(DisplayError::from)?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.display
            .clear()
            .map_err(DisplayError::from)
            .chain_err(DisplayError::ClearError)?;
        Ok(())
    }

    pub fn reset_pos(&mut self) -> Result<()> {
        self.set_position(0, 0)?;
        Ok(())
    }

    pub fn write_str(&mut self, s: &str) -> Result<()> {
        self.display.write_str(s).map_err(DisplayError::from)?;
        Ok(())
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<()> {
        self.display.write_fmt(args).map_err(DisplayError::from)?;
        Ok(())
    }

    pub fn set_position(&mut self, column: u8, row: u8) -> Result<()> {
        self.display
            .set_position(column, row)
            .map_err(DisplayError::from)?;
        Ok(())
    }
}

#[task]
pub async fn screen_counter(mut display: Display<DisplaySize128x64>) {
    let mut counter: u16 = 0;

    loop {
        Timer::after_millis(1).await;

        display.reset_pos().print_warn();
        display.write_fmt(format_args!("{}", counter)).print_warn();

        counter = match counter.checked_add(1) {
            Some(next) => next,
            None => {
                display.clear().print_warn();
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

        let success = display
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
        ));

        if let Err(e) = success {
            warn!("{e:?}");
            //try and just clear the display normally

            Backoff::new(|| display.clear())
                .with_log_level(log::Level::Trace)
                .with_max_elapsed_time(Duration::from_millis(500))
                .retry()
                .await
                .unwrap();
        };

        display.reset_pos().print_warn();
    }
}
