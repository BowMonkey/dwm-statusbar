use std::env;
use std::process::Command;

use utillib::util::{calc_notify_id, send_dwm, send_notify};

fn main() {
    let args_vec: Vec<String> = env::args().collect();
    if args_vec.len() > 2 {
        println!("Open paramater error! usage:date [L|R|M|U|D]");
        return;
    } else if args_vec.len() == 2 {
        match args_vec[1].as_str() {
            "L" | "l" => {
                left_click();
            }
            "R" | "r" => {
                right_click();
            }
            "M" | "m" => {
                middle_click();
            }
            "U" | "u" => {
                up_roll();
            }
            "D" | "d" => {
                down_rol();
            }
            _ => {
                println!(
                    "Operator Error! should be [L|R|M|U|D], received :{}",
                    args_vec[1]
                );
            }
        }
    }

    print!("{}", create_dwm_msg(""));
}

fn left_click() {}
fn right_click() {}
fn middle_click() {}
fn up_roll() {}
fn down_rol() {}

fn create_dwm_msg(s: &str) -> String {
    let title = "^s".to_string() + "icon" + "^";
    let ic_fg_color = "^c".to_string() + "#1e222a" + "^";
    let ic_bg_color = "^b".to_string() + "#A3BE8C" + "^";
    let ic = "".to_string();
    let tx_fg_color = "^c".to_string() + "#1e222a" + "^";
    let tx_bg_color = "^b".to_string() + "#A3BE8C" + "^";
    let tx = s.to_string();

    title + &ic_fg_color + &ic_bg_color + &ic + &tx_fg_color + &tx_bg_color + &tx
}

pub fn call_proc(cmd: &str) {
    Command::new("sh").arg("-c").arg(cmd).spawn().unwrap();
}
