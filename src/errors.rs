use crate::prelude::*;

#[derive(Clone, Copy, ErrorCategory)]
#[repr(u8)]
pub enum CansatError {
    I2C,
    Unknown,
    IntegerOverflow,
    InterfaceError,
}
