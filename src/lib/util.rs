use regex::Regex;

pub fn check_color(s: &String) -> bool {
    let re = Regex::new(r"^\#[0-9a-zA-Z]{6}").unwrap();
    re.is_match(s)
}

pub fn check_icon(s: &String) -> bool {
    let re = Regex::new(r"^\p{Greek}$").unwrap();
    re.is_match(s)
}

pub fn correct_time(num: &mut u32) {
    if *num == 0 {
        *num = 1;
    }
}
