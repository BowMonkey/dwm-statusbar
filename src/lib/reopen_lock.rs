use super::errors::*;

pub fn is_single() -> Result<bool> {
    let lock = named_lock::NamedLock::create("dwm-statusbar")?;
    let _guard = lock.try_lock();
    match _guard {
        Ok(_) => {
            return Ok(true);
        }
        Err(e) => match e {
            named_lock::Error::WouldBlock => {
                // double open, exit
                return Ok(false);
            }
            _ => {
                println!("Check reopen Error!");
                return Ok(true);
            }
        },
    }
}
