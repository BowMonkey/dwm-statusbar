use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Module {
    pub path_name: String,
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
            exec_interval: 1,
            max_exec_time: 1,
        }
    }
}
impl Default for Module {
    fn default() -> Self {
        Module {
            path_name: "".to_string(),
            exec_interval: 1,
            max_exec_time: 1,
        }
    }
}
