use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use utillib::config::*;
use utillib::scheduler::*;
use utillib::Errors;

fn main() -> Result<(), Errors> {
    // prevent double open
    let lock = named_lock::NamedLock::create("dwm-statusbar")?;
    let _guard = lock.try_lock();
    match _guard {
        Ok(_) => {
            // first time run
        }
        Err(e) => match e {
            named_lock::Error::WouldBlock => {
                //if called by dwm, parse args and call modules
                let args_vec: Vec<String> = env::args().collect();
                if args_vec.len() != 3 {
                    println!(
                        "Double open paramater error! usage:dwm-statusbar [module] [L|R|M|U|D]"
                    );
                    return Ok(());
                }
                let config = load_config().unwrap();
                if let Some(modul) = config.modules.into_iter().find(|item| {
                    let name = PathBuf::from(&item.path_name)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    name == args_vec[1]
                }) {
                    match Command::new(&modul.path_name).arg(&args_vec[2]).spawn() {
                        Ok(_) => {}
                        Err(e) => {
                            // fail is ok, wait next try
                            println!("Call proc failed. cmd:{} err:{}", modul.path_name, e);
                        }
                    }
                }

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
        println!("received Ctrl-C, set running flag to false;");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    run_scheduler(running);

    Ok(())
}
