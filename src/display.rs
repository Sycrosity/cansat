use crate::{
    mpu6050::{MpuSignal, MPU_SIGNAL},
    prelude::*,
};

use core::fmt::{self, Write};
use esp32_hal::{i2c::I2C, peripherals::I2C0};
use ssd1306::{
    mode::{TerminalDisplaySize, TerminalMode, TerminalModeError},
    prelude::*,
    I2CDisplayInterface, Ssd1306,
};

type DisplayInternals<SIZE> = Ssd1306<I2CInterface<I2CShared>, SIZE, TerminalMode>;

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
    #[error("{variant}: {summary}")]
    InterfaceError,
    ClearError,
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
        Self::WriteError
    }
}

pub struct Display<SIZE> {
    display: DisplayInternals<SIZE>,
}

impl<SIZE: DisplaySize + TerminalDisplaySize> Display<SIZE> {
    pub async fn new(i2c: I2CShared, display_size: SIZE) -> Result<Self> {
        let interface = I2CDisplayInterface::new(i2c);

        let display =
            Ssd1306::new(interface, display_size, DisplayRotation::Rotate0).into_terminal_mode();

        let mut display = Self { display };

        // display.display.init()?;

        display.try_init().await?;

        if let Err(e) = display.clear() {
            error!("{e:?}")
        };

        Ok(display)
    }

    fn init(&mut self) -> Result<()> {
        self.display.init().map_err(DisplayError::from)?;
        Ok(())
    }

    async fn try_init(&mut self) -> Result<()> {
        try_repeat(|| self.init(), DEFAULT_INTERVAL, DEFAULT_MAX_ELAPSED_TIME).await?;
        debug!("Initialised Display");
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.display
            .clear()
            .map_err(DisplayError::from)
            .chain_err(DisplayError::ClearError)?;
        Ok(())
    }

    pub fn quick_clear(&mut self) -> Result<()> {
        if let Err(e) = self.set_position(0, 0) {
            warn!("{e:?}");
            self.clear()?;
        };
        if let Err(e) = self.display.write_str("") {
            warn!("{e:?}");
            self.clear()?;
        };
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
    }
}
// farago
