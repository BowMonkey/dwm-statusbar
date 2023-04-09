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
    Command::new("sh")
        .arg("-c")
        .arg("notify-send")
        .arg(msg)
        .arg("-r")
        .arg(id.to_string())
        .spawn()
        .unwrap();
}

pub fn send_dwm(msg: &str) {
    let s = "xsetroot -name '".to_string() + msg + "'";
    if dwm_msg_ok(msg) {
        Command::new("sh").arg("-c").arg(s).spawn().unwrap();
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
