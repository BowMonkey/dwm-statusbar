use std::env;
use std::process::Command;

use utillib::util::{calc_notify_id, get_proc_info, send_dwm, send_notify};

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

    print!(
        "{}",
        create_dwm_msg("icon".to_string(), "".to_string(), "".to_string())
    );
}

use std::collections::HashMap;
fn call_rofi() {
    let mut cmds: HashMap<String, String> = HashMap::new();
    cmds.insert("󰍃 Logout".to_string(), "logout".to_string());
    cmds.insert("⏻ Shutdown".to_string(), "shutdown".to_string());
    cmds.insert(" Reboot".to_string(), "reboot".to_string());
    cmds.insert(" Lock".to_string(), "lock".to_string());

    let mut cmd_str = String::from("echo $(echo -e '");
    for key in cmds.keys() {
        cmd_str += key;
        cmd_str += "\\n";
    }
    cmd_str = cmd_str[0..cmd_str.len() - 2].to_string();
    cmd_str += "' | rofi -dmenu -window-title Power)";
    println!("cmd_str for rofi:{}", cmd_str);
    let selected = get_proc_info(&cmd_str, 20);
    if let Some(func) = cmds.get(&selected) {
        match func.as_str() {
            "logout" => do_logout(),
            "shutdown" => do_shutdown(),
            "reboot" => do_reboot(),
            "lock" => do_lock(),
            _ => {}
        }
    }
}
fn left_click() {
    call_rofi()
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

    title + &ic_fg_color + &ic_bg_color + " " + &ic + &tx_fg_color + &tx_bg_color + &tx + " "
}

fn call_proc(cmd: &str) {
    Command::new("sh").arg("-c").arg(cmd).spawn().unwrap();
}

fn do_logout() {
    call_proc("rm /tmp/dwm_avoid_repeat_auto_start.lock");
    call_proc("pkill -KILL -u coco")
}
fn do_shutdown() {
    call_proc("shutdown -P now")
}
fn do_reboot() {
    call_proc("reboot")
}
fn do_lock() {
    call_proc("slock")
}
