use regex::Regex;

pub fn check_color(s: &str) -> bool {
    let re = Regex::new(r"^\#[0-9a-zA-Z]{6}").unwrap();
    re.is_match(s)
}

pub fn correct_time(num: &mut u32) {
    if *num == 0 {
        *num = 1;
    }
}

pub fn dwm_msg_ok(s: &str) -> bool {
    //^s[window_name]^^c[icon_fg_color]^^b[icon_bg_color]0xff^[icon]^c[text_fg_color]^^b[text_bg_color]0xff^[TEXT_TO_SHOW]
    let re = Regex::new(r"^\^s[0-9a-zA-Z_]+\^\^c\#[0-9a-zA-Z]{6}\^\^b\#[0-9a-zA-Z]{6}\^.*?\^c\#[0-9a-zA-Z]{6}\^\^b\#[0-9a-zA-Z]{6}\^.*?").unwrap();
    re.is_match(s)
}

use std::process::Command;
pub fn send_notify(msg: &str, id: u32) {
    let s = "notify-send ".to_string() + msg + " -r " + &id.to_string();
    Command::new("sh").arg("-c").arg(s).spawn().unwrap();
}

pub fn send_dwm(msg: &str) {
    let s = "xsetroot -name '".to_string() + msg + "'";
    if dwm_msg_ok(msg) {
        match Command::new("sh")
            .arg("-c")
            .arg(" ".to_string() + &s + " ")
            .spawn()
        {
            Ok(_) => {}
            Err(e) => {
                //fail is ok, wait next try
                println!("Call xsetroot failed. cmd:{} err:{}", s, e);
            }
        }
    } else {
        println!("dwm msg check failure:{}", msg);
    }
}

pub fn calc_notify_id(s: &str) -> u32 {
    let mut res = 0;
    for ch in s.chars() {
        if ch.is_ascii() {
            res += ch as u32;
        }
    }
    res
}

use std::io::Read;
use std::process::Stdio;
use std::time::Duration;
use wait_timeout::ChildExt;
pub fn get_proc_info(cmd: &str, tm: u64) -> String {
    let mut childproc = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let secs = Duration::from_secs(tm);
    let _status_code = match childproc.wait_timeout(secs).unwrap() {
        Some(status) => status.code(),
        None => {
            childproc.kill().unwrap();
            childproc.wait().unwrap().code()
        }
    };
    let mut msg = String::new();
    childproc.stdout.unwrap().read_to_string(&mut msg).unwrap();

    msg.trim_end().to_string()
}
