use core::panic;

use utillib::{config::*, Errors, Module};

#[test]
fn create_and_save_config() {
    if Config::config_file_exist() {
        std::fs::remove_file(Config::config_file_path()).unwrap();
        let conf = load_config();
        match conf {
            Ok(mut cnf) => {
                cnf.modules.push(Module::example());
                println!("{:#?}", cnf.modules);
                save_config(&cnf).unwrap();
            }
            Err(e) => {
                panic!("Load config file error. error:{}", e);
            }
        }
        if !Config::config_file_exist() {
            panic!("save file error.");
        } else {
            println!(
                "config file path:{:#?}",
                Config::config_file_path().to_str()
            );
        }
    }
}

#[test]
fn read_from_config() {
    assert!(Config::config_file_exist());
    let conf = load_config().unwrap();
    println!("{:#?}", conf.modules);
}
