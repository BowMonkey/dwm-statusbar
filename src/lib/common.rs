use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DwmMsg {
    // process path_name with absolute path
    path_name: String,
    icon_fg_color: String,
    icon_bg_color: String,
    default_icon: String,
    text_fg_color: String,
    text_bg_color: String,
    default_text: String,
}

impl DwmMsg {
    pub fn new() -> Self {
        DwmMsg::default()
    }
}

impl Default for DwmMsg {
    fn default() -> Self {
        DwmMsg {
            path_name: "".to_string(),
            icon_fg_color: "".to_string(),
            icon_bg_color: "".to_string(),
            default_icon: "".to_string(),
            text_fg_color: "".to_string(),
            text_bg_color: "".to_string(),
            default_text: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Module {
    pub path_name: String,
    pub icon_fg_color: String,
    pub icon_bg_color: String,
    pub default_icon: String,
    pub text_fg_color: String,
    pub text_bg_color: String,
    pub default_text: String,
    // interval in seconds
    pub exec_interval: u32,
    // seconds
    pub max_exec_time: u32,
}
impl Module {
    pub fn new() -> Self {
        Module::default()
    }

    pub fn example() -> Self {
        let mut path = env::current_dir().unwrap();
        path.push("bin");
        path.push("Hello");

        Module {
            path_name: path.to_str().unwrap().to_string(),
            icon_fg_color: "#61afef".to_string(),
            icon_bg_color: "#2e323a".to_string(),
            default_icon: "".to_string(),
            text_fg_color: "#61afef".to_string(),
            text_bg_color: "#2e323a".to_string(),
            default_text: "Hello".to_string(),
            exec_interval: 1,
            max_exec_time: 1,
        }
    }
}
impl Default for Module {
    fn default() -> Self {
        Module {
            path_name: "".to_string(),
            icon_fg_color: "".to_string(),
            icon_bg_color: "".to_string(),
            default_icon: "".to_string(),
            text_fg_color: "".to_string(),
            text_bg_color: "".to_string(),
            default_text: "".to_string(),
            exec_interval: 1,
            max_exec_time: 1,
        }
    }
}
