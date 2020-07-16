use log::{Record, error, Metadata, LevelFilter,Level};
use std::panic;
use crate::runtimeapi::wapc::log;


/// Logger used for runtime purposes
/// 
/// Default to the Info level.
/// 
static LOGGER: RuntimeLogger = RuntimeLogger { level: Level::Trace };

struct RuntimeLogger {
    level: Level
}

/// Use the log crate for internal logging, and contract logging
/// 
/// following the example at https://docs.rs/log/0.4.8/log/fn.set_logger.html
impl log::Log for RuntimeLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            log(&format!("{} - {}", record.level(), record.args())[..]);
        }
    }

    fn flush(&self) {}
}

/// Called from the register contract macro.
/// 
/// Initalize the settings of the logger etc. 
pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
    
    // configure the panic hook, otherwise any panics
    // when running in Wasm will be lost
    panic::set_hook(Box::new(hook));
}


/// Hook function to capture the panic and route it 
/// to the logger
pub fn hook(info: &panic::PanicInfo) {
    let msg = info.to_string();

    // Finally, log the panic via waPC
    error!("[Panic]{}[/Panic]",msg);
}
