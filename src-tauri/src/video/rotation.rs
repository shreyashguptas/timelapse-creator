pub fn get_rotation_filter(rotation: u32) -> Option<String> {
    match rotation {
        0 => None,
        90 => Some("transpose=1".to_string()),
        180 => Some("transpose=1,transpose=1".to_string()),
        270 => Some("transpose=2".to_string()),
        _ => None,
    }
}
