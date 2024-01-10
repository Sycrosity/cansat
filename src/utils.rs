use crate::prelude::*;

#[inline]
pub async fn try_repeat<T: Copy, E: core::fmt::Debug + Copy, F: FnMut() -> Result<T, E>>(
    mut op: F,
    interval: Duration,
    max_elapsed_time: Duration,
) -> Result<T, E> {
    let mut counter = 1;

    let counter_limit = max_elapsed_time.as_ticks() / interval.as_ticks();

    let result: T = loop {
        match op() {
            success @ Ok(_) => 
                return success,
            
            Err(e) if counter <= counter_limit => {
                warn!("{e:?}: ({counter})");
                counter += 1;
                Timer::after(interval).await;
                continue;
            }
            error @ Err(e) => {
                error!("{e:?}");
                return error;
            }
        }
    };

    Ok(result)
}
