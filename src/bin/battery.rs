use std::env;

use utillib::util::{calc_notify_id,  send_notify};

use starship_battery::{
    units::{power::watt, ratio::percent, time::second},
    Battery, Manager, State,
};

 pub struct BatteryHarvest {
     charge_percent: f64,
     secs_until_full: Option<i64>,
     secs_until_empty: Option<i64>,
     power_consumption_rate_watts: f64,
     health_percent: f64,
     state: State,
}

fn main() {
    let mut click_event = false;
    let args_vec: Vec<String> = env::args().collect();
    if args_vec.len() > 2 {
        println!("Open paramater error! usage:date [L|R|M|U|D]");
        return;
    } else if args_vec.len() == 2 {
        click_event = true;
    }
    // 1. get battery info
    let mut list_of_batteries:Vec<BatteryHarvest>= Vec::new();
    if let Ok(battery_manager) = Manager::new() {
        if let Ok(batteries) = battery_manager.batteries() {
            let mut battery_list: Vec<Battery> = batteries.filter_map(Result::ok).collect();
            if !battery_list.is_empty() {
                list_of_batteries =
                    refresh_batteries(&battery_manager, &mut battery_list);
            }
        }
    }
    // 2. set default icon and default text
    let mut battery_text = "".to_string();
    let mut battery_icon = "󰂑".to_string();

    // 3. set icon and text accroading to current info
    if let Some(info) = list_of_batteries.first(){
        match info.state{
            State::Unknown => {
                battery_text = "100%".to_string();
                battery_icon = "󱟢".to_string();
            },
            State::Charging =>{
                battery_text = info.secs_until_full.unwrap_or(0).to_string() + "s";
                battery_icon = "󰂄".to_string();
            },
            State::Discharging =>{
                battery_icon = match info.secs_until_empty.unwrap_or(0){
                    20..=39 => "󰁻".to_string(),
                    40..=59 => "󰁽".to_string(),
                    60..=79 => "󰁿".to_string(),
                    80.. => "󰂁".to_string(),
                    _=> "󰂎".to_string(),
                };
                battery_text = info.secs_until_empty.unwrap_or(0).to_string() + "s";
            },
            State::Empty =>{
                battery_text = "0".to_string();
                battery_icon = "󰂃".to_string();
            },
            State::Full =>{
                battery_text = "100%".to_string();
                battery_icon = "󱟢".to_string();
            },
        }
    }

    if click_event {
        match args_vec[1].as_str() {
            "L" | "l" => {
                let msg = r#"""#.to_string() + &battery_icon + r#" Battery" ""# + &battery_text + r#"""#;
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
        print!(
            "{}",
            create_dwm_msg("Battery".to_string(), battery_icon, battery_text)
        );
    }
}

fn left_click(msg: String) {
    send_notify(&msg, calc_notify_id("Battery"));
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


pub fn refresh_batteries(manager: &Manager, batteries: &mut [Battery]) -> Vec<BatteryHarvest> {
    batteries
        .iter_mut()
        .filter_map(|battery| {
            if manager.refresh(battery).is_ok() {
                Some(BatteryHarvest {
                    secs_until_full: {
                        let optional_time = battery.time_to_full();
                        optional_time.map(|time| f64::from(time.get::<second>()) as i64)
                    },
                    secs_until_empty: {
                        let optional_time = battery.time_to_empty();
                        optional_time.map(|time| f64::from(time.get::<second>()) as i64)
                    },
                    charge_percent: f64::from(battery.state_of_charge().get::<percent>()),
                    power_consumption_rate_watts: f64::from(battery.energy_rate().get::<watt>()),
                    health_percent: f64::from(battery.state_of_health().get::<percent>()),
                    state: battery.state(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}


