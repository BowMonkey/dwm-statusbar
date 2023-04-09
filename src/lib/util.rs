use regex::Regex;

pub fn check_color(s: &str) -> bool {
    let re = Regex::new(r"^\#[0-9a-zA-Z]{6}").unwrap();
    re.is_match(s)
}

pub fn correct_time(num: &mut u32) {
    if *num == 0 {
        *num = 1;
    }
}

pub fn check_dwm_msg(s: &str) -> bool {
    //^s[window_name]^^c[icon_fg_color]^^b[icon_bg_color]0xff^[icon]^c[text_fg_color]^^b[text_bg_color]0xff^[TEXT_TO_SHOW]
    let re = Regex::new(r"^\^s[0-9a-zA-Z_]+\^\^c\#[0-9a-zA-Z]{6}\^\^b\#[0-9a-zA-Z]{6}\^.*?\^c\#[0-9a-zA-Z]{6}\^\^b\#[0-9a-zA-Z]{6}\^.*?").unwrap();
    re.is_match(s)
}
