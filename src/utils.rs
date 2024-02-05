use crate::prelude::*;

pub struct Backoff<T, E, OP>
where
    T: Copy,
    E: core::fmt::Debug,
    OP: FnMut() -> Result<T, E>,
{
    ///An optional log level to print the error on each retry.
    ///
    /// Defaults to [None].
    log_level: Option<log::Level>,
    /// The function to be ran on each repeat
    ///
    /// Must return a [Result].
    op: OP,
    /// How long to wait between retries.
    ///
    /// Defaults to 500ms.
    interval: Duration,
    /// Whether to have a maximum allowed time before returning an error, and how long for.
    ///
    /// Defaults to 5 seconds.
    max_elapsed_time: Option<Duration>,
}

impl<T, E, OP> Backoff<T, E, OP>
where
    T: Copy,
    E: core::fmt::Debug,
    OP: FnMut() -> Result<T, E>,
{
    pub fn new(op: OP) -> Self {
        Self {
            log_level: Some(log::Level::Trace),
            op,
            interval: Duration::from_millis(500),
            max_elapsed_time: Some(Duration::from_secs(5)),
        }
    }

    pub fn with_log_level(mut self, log_level: impl Into<Option<log::Level>>) -> Self {
        self.set_log_level(log_level.into());
        self
    }

    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.set_interval(interval);
        self
    }

    pub fn with_max_elapsed_time(mut self, max_elapsed_time: impl Into<Option<Duration>>) -> Self {
        self.set_max_elapsed_time(max_elapsed_time.into());
        self
    }

    pub fn set_log_level(&mut self, log_level: impl Into<Option<log::Level>>) {
        self.log_level = log_level.into();
    }
    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }
    pub fn set_max_elapsed_time(&mut self, max_elapsed_time: impl Into<Option<Duration>>) {
        self.max_elapsed_time = max_elapsed_time.into();
    }

    pub async fn retry(&mut self) -> Result<T, E> {
        let mut counter = 1;

        let counter_limit =
            self.max_elapsed_time.unwrap_or(Duration::MAX).as_ticks() / self.interval.as_ticks();

        loop {
            match (self.op)() {
                success @ Ok(_) => return success,

                Err(e) if counter <= counter_limit => {
                    if let Some(level) = self.log_level {
                        log!(level, "{e:?}: ({counter})");
                    }

                    counter += 1;
                    Timer::after(self.interval).await;
                    continue;
                }
                error @ Err(_) => {
                    return error;
                }
            }
        }
    }
}

pub trait PrintErr<T: Default> {
    fn print_warn(self) -> T;
    fn print_error(self) -> T;
    fn print_trace(self) -> T;
    fn print_debug(self) -> T;
    fn print_info(self) -> T;

    fn print_log(self, log_level: log::Level) -> T;
}

impl<T, E> PrintErr<T> for Result<T, E>
where
    T: Default,
    E: core::fmt::Debug,
{
    fn print_warn(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                warn!("{e:?}");
                T::default()
            }
        }
    }

    fn print_error(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                error!("{e:?}");
                T::default()
            }
        }
    }

    fn print_trace(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                trace!("{e:?}");
                T::default()
            }
        }
    }

    fn print_debug(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                debug!("{e:?}");
                T::default()
            }
        }
    }

    fn print_info(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                info!("{e:?}");
                T::default()
            }
        }
    }

    fn print_log(self, log_level: log::Level) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                log!(log_level, "{e:?}");
                T::default()
            }
        }
    }
}
