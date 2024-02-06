macro_rules! CSI {
    () => {
        "\x1B["
    };
}
pub fn clear_screen() {
    let combin = format!("{}{}", CSI!(), "2J");
    print!("{}", combin);
}
pub fn reset_color() {
    let combin = format!("{}{}", CSI!(), "0m");
    print!("{}", combin);
}
pub fn hide_cursor() {
    let combin = format!("{}{}", CSI!(), "?25l");
    print!("{}", combin);
}
pub fn show_cursor() {
    let combin = format!("{}{}", CSI!(), "?25h");
    print!("{}", combin);
}
pub fn set_fore_color(color: String) {
    let combin = format!("{}{}{}{}", CSI!(), "38;5;", color, "m");
    print!("{}", combin);
}
pub fn set_back_color(color: u32) {
    let combin = format!("{}{}{}{}", CSI!(), "48;5;", color, "m");
    print!("{}", combin);
}
pub fn move_cursor_to(x: u32, y: u32) {
    let combin = format!("{}{}{}{}{}", CSI!(), x, ";", y, "H");
    print!("{}", combin);
}
