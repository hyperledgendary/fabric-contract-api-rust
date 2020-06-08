use log::{Record, error, Metadata, LevelFilter};

use crate::runtimeapi::wapc::log;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // metadata.level() <= Level::Info
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            log(&format!("{} - {}", record.level(), record.args())[..]);
        }
    }

    fn flush(&self) {}
}


use std::panic;

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
    
    panic::set_hook(Box::new(hook));
}

pub fn hook(info: &panic::PanicInfo) {
    let msg = info.to_string();

    // nightly rust builds can produce the stack trace
    // msg.push_str("\n\nStack:\n\n");
    // let e = error::new();
    // let stack = e.stack();
    // msg.push_str(&stack);

    // Finally, log the panic via waPC
    error!("{}",msg);
}
