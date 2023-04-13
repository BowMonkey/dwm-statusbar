use std::env;
use std::process::Command;

use utillib::util::{calc_notify_id, get_proc_info, send_dwm, send_notify};

fn main() {
    let mut click_event = false;
    let args_vec: Vec<String> = env::args().collect();
    if args_vec.len() > 2 {
        println!("Open paramater error! usage:date [L|R|M|U|D]");
        return;
    } else if args_vec.len() == 2 {
        click_event = true;
    }

    let status = get_proc_info("playerctl metadata -p chromium --format '{{ status }}'", 1);
    let mut icon = String::new();
    if status == "Playing".to_string() {
        icon = "󰏤".to_string();
    } else if status == "Paused" {
        icon = "".to_string();
    } else {
        icon = "".to_string();
    }

    let mut play_text = get_proc_info(
        "playerctl metadata -p chromium --format '{{ artist }}-{{ title }}'",
        1,
    );
    if play_text == "No players found" {
        play_text = "".to_string();
    }

    if click_event {
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
    } else {
        print!(
            "{}",
            create_dwm_msg("MusicPlay".to_string(), icon, play_text)
        );
    }
}

fn left_click() {
    call_proc("playerctl play-pause -p chromium");
}
fn right_click() {}
fn middle_click() {}
fn up_roll() {}
fn down_rol() {}

fn create_dwm_msg(title: String, icon: String, text: String) -> String {
    let title = "^s".to_string() + &title + "^";
    let ic_fg_color = "^c".to_string() + "#1e222a" + "^";
    let ic_bg_color = "^b".to_string() + "#81A1C1" + "^";
    let ic = icon;
    let tx_fg_color = "^c".to_string() + "#1e222a" + "^";
    let tx_bg_color = "^b".to_string() + "#81A1C1" + "^";
    let tx = text;

    title + &ic_fg_color + &ic_bg_color + &ic + &tx_fg_color + &tx_bg_color + tx.trim_end()
}

pub fn call_proc(cmd: &str) {
    Command::new("sh").arg("-c").arg(cmd).spawn().unwrap();
}
