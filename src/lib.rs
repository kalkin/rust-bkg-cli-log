//! A logger which logs to `STDERR` by default but to `STDOUT` on `info!()`.
use log::{Level, LevelFilter, Log, Metadata, Record};

use std::io::{self, Write};

static LOGGER: CliLog = CliLog {
    always_stderr: false,
};

static STDERR_LOGGER: CliLog = CliLog {
    always_stderr: true,
};

struct CliLog {
    always_stderr: bool,
}

#[allow(clippy::print_stdout, clippy::print_stderr)]
impl Log for CliLog {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() >= log::max_level()
    }

    fn log(&self, record: &Record<'_>) {
        if record.level() <= log::max_level() {
            match record.level() {
                Level::Error => eprintln!("error: {}", record.args()),
                Level::Warn => {
                    if self.always_stderr {
                        eprintln!("warn: {}", record.args());
                    } else {
                        println!("{}", record.args());
                    }
                }
                Level::Info => {
                    if self.always_stderr {
                        eprintln!("{}", record.args());
                    } else {
                        println!("{}", record.args());
                    }
                }
                Level::Debug => eprintln!("debug: {}", record.args()),
                Level::Trace => eprintln!("trace: {}", record.args()),
            }
        }
    }

    #[allow(unused_results, unused_must_use)]
    fn flush(&self) {
        io::stdout().flush();
        io::stderr().flush();
    }
}

/// Initialize logger with specified `LevelFilter`
#[inline]
pub fn init_with_level(filter: LevelFilter) {
    log::set_logger(&LOGGER).expect("Setup logger");
    log::set_max_level(filter);
}

/// Initialize a logger printing to `STDERR` with specified `LevelFilter`
#[inline]
pub fn init_stderr_with_level(filter: LevelFilter) {
    log::set_logger(&STDERR_LOGGER).expect("Setup logger");
    log::set_max_level(filter);
}
