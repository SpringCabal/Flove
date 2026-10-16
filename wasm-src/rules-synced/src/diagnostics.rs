pub const LOG_SECTION: &str = "flove-rules";

macro_rules! trace {
    ($($arg:tt)*) => {
        if cfg!(feature = "diagnostics") {
            spring::log_info!($crate::diagnostics::LOG_SECTION, $($arg)*);
        }
    };
}
pub(crate) use trace;

pub fn check<T>(
    result: spring::Result<T>,
    what: impl FnOnce() -> alloc::string::String,
) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            spring::log_warning!(LOG_SECTION, "{} failed: {error:?}", what());
            None
        }
    }
}
