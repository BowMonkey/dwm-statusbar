use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use utillib::Errors;

fn main() -> Result<(), Errors> {
    // prevent double open
    let lock = named_lock::NamedLock::create("dwm-statusbar")?;
    let _guard = lock.try_lock();
    match _guard {
        Ok(_) => {
            // first run
        }
        Err(e) => match e {
            named_lock::Error::WouldBlock => {
                // double open, exit
                return Ok(());
            }
            _ => {
                println!("Other Error!");
                return Err(Errors::DoubleOpenError(e));
            }
        },
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        //do things
    }

    // time to die, clean up
    // reset dwm statusbar send: xsetroot -name ''
    Ok(())
}
