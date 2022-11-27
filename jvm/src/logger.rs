use crate::log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

extern crate proc_macro;
struct Logger;

#[macro_export]
macro_rules! exit {
    ($($arg:tt)+) => {
        {
            ::log::error!("{}", format!($($arg)+));
            std::process::exit(1)
        }
    }
}

static LOGGER: Logger = Logger;
impl log::Log for Logger {
    fn enabled(&self, met: &Metadata) -> bool {
        #[cfg(debug_assertions)]
        return met.level() <= Level::Trace;

        #[cfg(not(debug_assertions))]
        return met.level() <= Level::Error;
    }

    fn log(&self, rec: &Record) {
        if self.enabled(rec.metadata()) {
            eprintln!(
                "[{}] {}:{} {}",
                rec.level(),
                rec.file().unwrap(),
                rec.line().unwrap(),
                rec.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn init(lf: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(lf))
}
