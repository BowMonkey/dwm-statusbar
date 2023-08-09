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

    let mut icon = match get_proc_info("cat /sys/class/net/w*/carrier", 1 as u64).as_str() {
        "" => "󰤬".to_string(),  //device is turned off
        "0" => "󰖪".to_string(), //wifi down
        "1" => "󰖩".to_string(), //wifi up
        __ => "󰤩".to_string(),  // undefined
    };
    let wifi_signal = get_proc_info(
        r#"echo $(awk '/^\s*w/ { print int($3 * 100 / 70)}' /proc/net/wireless)"#,
        1 as u64,
    );
    icon = match wifi_signal.parse::<u32>() {
        Ok(level) => match level {
            0..=19 => "󰤯".to_string(),
            20..=39 => "󰤟".to_string(),
            40..=59 => "󰤢".to_string(),
            60..=79 => "󰤥".to_string(),
            80.. => "󰤨".to_string(),
        },
        _ => "󰤩".to_string(),
    };
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
            create_dwm_msg("wifi".to_string(), icon, "".to_string())
        );
    }
}

fn left_click() {
    call_proc("nm-connection-editor");
}
fn right_click() {
    call_proc("alacritty -t nmtui --class floatingTerminal -e nmtui ");
}
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

    title + &ic_fg_color + &ic_bg_color + " " +  &ic + &tx_fg_color + &tx_bg_color + &tx + " "
}

pub fn call_proc(cmd: &str) {
    Command::new("sh").arg("-c").arg(cmd).spawn().unwrap();
}
