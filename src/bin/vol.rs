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

    let sink = get_proc_info(
        "echo $(pactl info | grep 'Default Sink' | awk '{print $3}')",
        1 as u64,
    );
    let cmd = "echo $(pactl list sinks | grep ".to_string()
        + &sink
        + " -A 6 | sed -n '7p' | grep 'Mute: no')";
    let volumuted = get_proc_info(&cmd, 1);

    let cmd = "echo $(pactl list sinks | grep ".to_string()
        + &sink
        + " -A 7 | sed -n '8p' | awk '{printf int($5)}' )";
    let volume = get_proc_info(&cmd, 1);

    let mut vol_text = "--".to_string();
    let mut vol_icon = "".to_string();
    if volumuted.is_empty() {
        vol_text = "--".to_owned();
        vol_icon = "ﱝ".to_owned();
    } else {
        (vol_icon, vol_text) = match volume.parse::<i32>() {
            Ok(v) if v > 0 => ("".to_string(), volume),
            Ok(v) if v == 0 => ("".to_string(), "00".to_string()),
            _ => ("ﱝ".to_string(), "Err".to_string()),
        };
    }
    if click_event {
        match args_vec[1].as_str() {
            "L" | "l" => {
                let msg = r#"""#.to_string() + &vol_icon + r#" Volume" ""# + &vol_text + r#"""#;
                left_click(msg);
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
        print!("{}", create_dwm_msg(vol_icon, vol_text));
    }
}

fn left_click(msg: String) {
    send_notify(&msg, calc_notify_id("vol"));
}
fn right_click() {
    call_proc("killall pavucontrol ||  pavucontrol --class floatingTerminal &");
}
fn middle_click() {
    call_proc("pactl set-sink-mute @DEFAULT_SINK@ toggle");
    call_proc("/home/coco/Desktop/dwm/statusbar/bin/vol L")
}
fn up_roll() {
    call_proc("pactl set-sink-volume @DEFAULT_SINK@ +5%");
    call_proc("/home/coco/Desktop/dwm/statusbar/bin/vol L")
}
fn down_rol() {
    call_proc("pactl set-sink-volume @DEFAULT_SINK@ -5%");
    call_proc("/home/coco/Desktop/dwm/statusbar/bin/vol L")
}

fn create_dwm_msg(icon: String, text: String) -> String {
    let title = "^s".to_string() + "vol" + "^";
    let ic_fg_color = "^c".to_string() + "#1e222a" + "^";
    let ic_bg_color = "^b".to_string() + "#81A1C1" + "^";
    let ic = icon;
    let tx_fg_color = "^c".to_string() + "#1e222a" + "^";
    let tx_bg_color = "^b".to_string() + "#81A1C1" + "^";
    let tx = text;

    title + &ic_fg_color + &ic_bg_color + " " + &ic + &tx_fg_color + &tx_bg_color + &tx + " "
}

pub fn call_proc(cmd: &str) {
    Command::new("sh").arg("-c").arg(cmd).spawn().unwrap();
}
