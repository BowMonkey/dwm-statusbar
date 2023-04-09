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
            snd.send((idx, item.clone())).unwrap();
        });
    }

    /****************************************
     * 3. spawn a thread, in this thread, we create work thread pool.
     * when a task is called by scheduler, work thread pool will chose a
     * thread to run it in timeout time.
     ****************************************/
    //channel between worker thread pool and main thread
    let (dwm_sender, dwm_recver) = channel();

    let running_flag = running.clone();
    let worker_join_handle = thread::spawn(move || {
        let pool = ThreadPool::new(4);
        while running_flag.load(Ordering::SeqCst) {
            if let Ok((idx, val)) = rcver.try_recv() {
                let dwm_sender_in_thread = dwm_sender.clone();
                pool.execute(move || {
                    let mut childproc = Command::new("sh")
                        .arg("-c")
                        .arg(&val.path_name)
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();
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
                    println!("msg from childproc:{}", msg);
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
                })
            }
            thread::sleep(Duration::from_millis(10));
        }
        pool.join();
    });

    /****************************************
     * 4. update result and send messages to dwm
     ****************************************/
    let scheduler_handle = scheduler.watch_thread(Duration::from_millis(500));
    let mut dwm_msgs = vec!["".to_string(); config_num];
    //refresh dwm every 200ms
    while running.load(Ordering::SeqCst) {
        if let Ok((idx, msg)) = dwm_recver.try_recv() {
            dwm_msgs[idx] = msg;
            let mut msg = String::from("xsetroot -name '");
            for item in dwm_msgs.iter() {
                msg += item;
            }
            msg += "'";
            send_to_dwm(msg);
        }
        thread::sleep(Duration::from_millis(200));
    }
    scheduler_handle.stop();
    worker_join_handle.join().unwrap();
    send_to_dwm(last_words());
}
fn last_words() -> String {
    String::from("xsetroot -name ''")
}

fn send_to_dwm(s: String) {
    println!("Msg to be send:{}", s);
    let mut _childproc = Command::new("sh")
        .arg("-c")
        .arg(s)
        .spawn()
        .expect("Faild to call xsetroot command");
}
