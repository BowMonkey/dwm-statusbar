use utillib::util::*;

#[test]
fn test_dwm_msg1() {
    let s = "^snet^^c#81A1C1^^b#1e222a^^c#81A1C1^^b#1e222a^   497B    885B".to_string();
    assert!(check_dwm_msg(&s));
}

#[test]
fn test_dwm_msg2() {
    let s = "^sn-et^^c#81A1C1^^b#1e222a^^c#81A1C1^^b#1e222a^   497B    885B".to_string();
    assert!(!check_dwm_msg(&s));
}
#[test]
fn test_dwm_msg3() {
    let s = "^snet^^c#81A1C1^^b#1e222a^^c#81A1C1^^b#1e222a^   497B    885B".to_string();
    assert!(check_dwm_msg(&s))
}
#[test]
fn test_dwm_msg4() {
    let s = "^snet^^c#81A1C1^^b#1e222a^123  ^c#81A1C1^^b#1e222a^   497B    885B".to_string();
    assert!(check_dwm_msg(&s));
}
#[test]
fn test_dwm_msg5() {
    let s = "^snet^^c#81A1C1^^b#1e222a^  ^c#81A1C1^^b#1e222a^   497B    885B".to_string();
    assert!(check_dwm_msg(&s));
}

#[test]
fn test_dwm_msg6() {
    let s = "^snet^c#81A1C1^^b#1e222a^  ^c#81A1C1^^b#1e222a^   497B    885B".to_string();
    assert!(!check_dwm_msg(&s));
}

#[test]
fn test_dwm_msg7() {
    let s = "^sdate^^c#ff0000^^b#00ff00^^c#ff0000^^b#ff0000^date".to_string();
    assert!(check_dwm_msg(&s));
}

#[test]
fn test_dwm_msg8() {
    let s = "^sdate^^c#ff0000^^b#ffffff^^c#ff0000^^b#ffffff^date".to_string();
    assert!(check_dwm_msg(&s));
}
