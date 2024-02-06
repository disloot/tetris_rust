mod terminal;
use std::{io::Write, time::Duration};
use terminal as tc;
fn main() {
    tc::hide_cursor();
    for i in 0..10 {
        tc::clear_screen();
        tc::move_cursor_to(i, 10);
        tc::set_back_color(15);
        print!("   ");
        std::io::stdout().flush().unwrap();
        tc::reset_color();
        std::thread::sleep(Duration::from_millis(500));
    }
    tc::show_cursor();
}
