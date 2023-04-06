use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CallType {
    // send process output to dwm directly
    Direct,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DwmMsg {
    // process name will be call
    window_name: String,
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
            window_name: "".to_string(),
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
    window_name: String,
    icon_fg_color: String,
    icon_bg_color: String,
    default_icon: String,
    text_fg_color: String,
    text_bg_color: String,
    default_text: String,
    // module position you want to show, might not success
    order: u32,
    // interval in seconds
    exec_interval: u32,
    // call type
    call_type: CallType,
}
impl Module {
    pub fn new() -> Self {
        Module::default()
    }

    pub fn example() -> Self {
        Module {
            window_name: "Hello".to_string(),
            icon_fg_color: "#61afef".to_string(),
            icon_bg_color: "#2e323a".to_string(),
            default_icon: "".to_string(),
            text_fg_color: "#61afef".to_string(),
            text_bg_color: "#2e323a".to_string(),
            default_text: "Hello".to_string(),
            order: 0,
            exec_interval: 1,
            call_type: CallType::Direct,
        }
    }
}
impl Default for Module {
    fn default() -> Self {
        Module {
            window_name: "".to_string(),
            icon_fg_color: "".to_string(),
            icon_bg_color: "".to_string(),
            default_icon: "".to_string(),
            text_fg_color: "".to_string(),
            text_bg_color: "".to_string(),
            default_text: "".to_string(),
            order: 0,
            exec_interval: 1,
            call_type: CallType::Direct,
        }
    }
}
