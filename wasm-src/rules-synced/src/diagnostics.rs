pub const LOG_SECTION: &str = "flove-rules";

macro_rules! trace {
    ($spring:expr, $($arg:tt)*) => {
        if cfg!(feature = "diagnostics") {
            spring::log_info!($spring, $crate::diagnostics::LOG_SECTION, $($arg)*);
        }
    };
}
pub(crate) use trace;

pub fn check<T>(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    result: spring::Result<T>,
    what: impl FnOnce() -> alloc::string::String,
) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            spring::log_warning!(spring, LOG_SECTION, "{} failed: {error:?}", what());
            None
        }
    }
}
