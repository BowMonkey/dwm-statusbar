use utillib::util::*;

#[test]
fn test_dwm_msg1() {
    let s = "^snet^^c#81A1C1^^b#1e222a0xff^^c#81A1C1^^b#1e222a0xff^   497B    885B".to_string();
    assert!(check_dwm_msg(&s));
}

#[test]
fn test_dwm_msg2() {
    let s = "^sn-et^^c#81A1C1^^b#1e222a0xff^^c#81A1C1^^b#1e222a0xff^   497B    885B".to_string();
    assert!(!check_dwm_msg(&s));
}
#[test]
fn test_dwm_msg3() {
    let s = "^snet^^c#81A1C1^^b#1e222a0xff^^c#81A1C1^^b#1e222a0xff^   497B    885B".to_string();
    assert!(check_dwm_msg(&s))
}
#[test]
fn test_dwm_msg4() {
    let s =
        "^snet^^c#81A1C1^^b#1e222a0xff^123  ^c#81A1C1^^b#1e222a0xff^   497B    885B".to_string();
    assert!(check_dwm_msg(&s));
}
#[test]
fn test_dwm_msg5() {
    let s =
        "^snet^^c#81A1C1^^b#1e222a0xff^  ^c#81A1C1^^b#1e222a0xff^   497B    885B".to_string();
    assert!(check_dwm_msg(&s));
}

#[test]
fn test_dwm_msg6() {
    let s = "^snet^c#81A1C1^^b#1e222a0xff^  ^c#81A1C1^^b#1e222a0xff^   497B    885B".to_string();
    assert!(!check_dwm_msg(&s));
}
