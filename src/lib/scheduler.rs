use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use clokwerk::{Scheduler, TimeUnits};
use threadpool::ThreadPool;
use wait_timeout::ChildExt;

use crate::dwm_msg_ok;

use super::config::*;
pub fn run_scheduler(running: Arc<AtomicBool>) {
    /****************************************
     * 1. read config file
     ****************************************/
    let config = load_config().unwrap();
    let config_num = config.modules.len();

    /****************************************
     * 2. add tasks to scheduler
     ****************************************/
    //channel between scheduler and worker thread pool
    let (snder, rcver) = channel();
    // and tasks to scheduler
    let mut scheduler = Scheduler::new();
    for (idx, item) in config.modules.into_iter().enumerate() {
        let snd = snder.clone();
        scheduler.every(item.exec_interval.seconds()).run(move || {
            //send msg in channal, so tasks will not be blocked
            snd.send((idx, item.clone())).unwrap();
        });
    }
    let scheduler_handle = scheduler.watch_thread(Duration::from_millis(100));

    //channel between worker thread pool and main thread
    let (dwm_sender, dwm_recver) = channel();
    let mut dwm_msgs = vec!["".to_string(); config_num];
    let pool = ThreadPool::new(4);
    while running.load(Ordering::SeqCst) {
        while let Ok((idx, val)) = rcver.try_recv() {
            let dwm_sender_in_thread = dwm_sender.clone();
            pool.execute(move || {
                if let Ok(mut childproc) = Command::new("sh")
                    .arg("-c")
                    .arg(&val.path_name)
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    let secs = Duration::from_secs(val.max_exec_time as u64);
                    let _status_code = match childproc.wait_timeout(secs).unwrap() {
                        Some(status) => status.code(),
                        None => {
                            childproc.kill().unwrap();
                            childproc.wait().unwrap().code()
                        }
                    };
                    let mut msg = String::new();
                    childproc
                        .stdout
                        .unwrap()
                        .read_to_string(&mut msg)
                        .unwrap()
                        .to_string();
                    if !dwm_msg_ok(&msg) {
                        let name = val.path_name.clone();
                        let name = PathBuf::from(name)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                        msg = "^s".to_owned()
                            + &name
                            + "^^c#ff0000^^b#ffffff^^c#ff0000^^b#ffffff^"
                            + &name;
                    }
                    dwm_sender_in_thread.send((idx, msg)).unwrap();
                }
            })
        }
        thread::sleep(Duration::from_millis(100));
        /****************************************
         *  update result and send messages to dwm
         ****************************************/
        while let Ok((idx, msg)) = dwm_recver.try_recv() {
            dwm_msgs[idx] = msg;
        }
        let mut msg = String::from("xsetroot -name \"");
        for item in dwm_msgs.iter() {
            msg += item;
        }
        msg += "\"";
        send_to_dwm(msg);
    }

    scheduler_handle.stop();
    pool.join();
    send_to_dwm(last_words());
}
fn last_words() -> String {
    String::from("xsetroot -name ''")
}

fn send_to_dwm(s: String) {
    match Command::new("sh").arg("-c").arg(s).spawn() {
        Ok(_) => {}
        Err(e) => {
            println!("Call xsetroot failed! err:{}", e);
        }
    }
}
