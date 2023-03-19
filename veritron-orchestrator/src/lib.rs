pub mod cli;
pub mod errors;
pub mod simple;

use errors::OrchestrationError;
use std::{
    backtrace::{Backtrace, BacktraceStatus},
    collections::HashMap,
    panic, process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::current,
};

pub fn set_panic_hook() {
    panic::set_hook(Box::new(move |panic_info| {
        // All the orchestrator errors are captured here
        if let Some(e) = panic_info.payload().downcast_ref::<OrchestrationError>() {
            println!("{}", e);
        } else {
            println!("{}", panic_info);
        }

        let backtrace = Backtrace::capture();
        if backtrace.status() == BacktraceStatus::Captured {
            println!(
                "thread '{}' panicked at '{}'\n stack backtrace:\n{}",
                current()
                    .name()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                panic_info
                    .location()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                backtrace
            );
        }

        process::exit(1);
    }));
}

pub fn set_ctrl_handler(r: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
}

pub trait Orchestrator {
    fn run(&self) -> Result<(), OrchestrationError>;
}
